//! Code-Mode-aware response budgeting.
//!
//! The Code Mode envelope (`{result, calls, logs, artifacts, …}`) can blow an
//! agent's context window. yarr's transport layer already caps the whole MCP
//! tool response at [`crate::token_limit::MAX_RESPONSE_BYTES`] — but that cap is a
//! *blind byte slice* that chops `result` mid-JSON and discards the audit envelope
//! structure. This pass shapes the envelope INTELLIGENTLY, strictly BELOW the
//! transport cap, so the agent always receives parseable JSON:
//!
//! Sacrifice order is value-ordered — preserve `result` (the answer), then the
//! `calls` audit, then artifact receipts, then `logs` (debug output):
//!
//! 1. If the serialized envelope already fits the budget → leave it untouched.
//! 2. Otherwise set artifact receipts and `logs` aside — the script's answer is
//!    more valuable than its auxiliary output.
//!    (yarr diverges from lab here, which caps the result first.)
//! 3. If that non-log payload is itself still over budget, replace an oversized
//!    `result` with a structured, parseable
//!    `{truncated, original_bytes, original_tokens, preview, next_action}` marker
//!    — a result-specific marker in the same *spirit* as `token_limit`'s
//!    `{truncated, reason, partial}` (both lead with `truncated: true` so an agent
//!    can branch programmatically), but a distinct shape — then, if STILL over,
//!    trim the `calls` audit newest-first with a `{truncated_calls: N}` sentinel.
//! 4. Fit back newest artifact receipts, then as many of the newest `logs` lines
//!    as the remaining budget allows, each with an explicit truncation sentinel.
//!
//! The budget is *derived from* `MAX_RESPONSE_BYTES` (3/5 of it ≈ 24 KB, the same
//! figure lab and Cloudflare's codemode use) so the two caps can never invert and
//! the shaped envelope never trips the transport truncation that would undo this
//! work. Token counts in the marker are informational (bytes / 4, a conservative
//! over-estimate for JSON); the budget *check* is bytes, matching the transport cap.

use serde_json::{Value, json};

use crate::token_limit::MAX_RESPONSE_BYTES;

/// Byte budget for the shaped Code Mode envelope. 3/5 of the transport cap leaves
/// generous headroom (~16 KB at the default 40 KB cap) so re-serialization /
/// escape growth can never push the shaped envelope past the transport truncation.
pub(crate) const RESPONSE_BUDGET: usize = MAX_RESPONSE_BYTES / 5 * 3;
pub(crate) const FLEET_RESULT_BUDGET: usize = RESPONSE_BUDGET - 2 * 1024;

// The whole point is to shape the envelope BELOW the transport cap; pin that
// invariant at compile time so a future change to either constant can't invert it.
const _: () = assert!(
    RESPONSE_BUDGET < MAX_RESPONSE_BYTES,
    "Code Mode budget must stay below the transport cap"
);

/// Bytes-per-token estimate for the (informational) token count in the marker.
const TOKEN_DIVISOR: usize = 4;

/// Head-preview size (bytes) of an oversized result placed in the marker.
const PREVIEW_BYTES: usize = 1024;

/// Shape `response` to fit [`RESPONSE_BUDGET`] in place (see module docs).
///
/// Sacrifice order is value-ordered: preserve `result` (the answer) first, then the
/// `calls` audit, then `logs` (debug output) — each trimmed only as far as needed.
pub fn fit_response(response: &mut Value) {
    if within_budget(response) {
        return;
    }

    summarize_fleet_result(response);
    if within_budget(response) {
        return;
    }

    // Pull lowest-value metadata out so result and call receipts are budgeted first.
    let logs = take_array(response, "logs");
    let artifacts = take_array(response, "artifacts");

    if !within_budget(response) {
        // The non-log payload (result/calls/artifacts) is itself over budget, so
        // trimming logs alone can't help — replace an oversized `result` with a
        // compact marker (only when that actually shrinks it).
        marker_oversized_result(response);
    }

    if !within_budget(response) {
        // Still over with logs gone and the result markered → the `calls` audit is
        // the bottleneck (e.g. many failing calls with verbose error bodies). Trim
        // it newest-first with a `{truncated_calls: N}` sentinel.
        let calls = take_array(response, "calls");
        fit_newest(
            response,
            "calls",
            calls,
            |dropped| json!({ "truncated_calls": dropped }),
        );
    }

    fit_newest(
        response,
        "artifacts",
        artifacts,
        |dropped| json!({ "truncated_artifacts": dropped }),
    );

    // Finally, fit back as many of the NEWEST log lines as the remaining budget allows.
    fit_newest(response, "logs", logs, |dropped| {
        Value::String(format!(
            "[logs truncated to fit response budget — {dropped} line(s) dropped]"
        ))
    });
    if !within_budget(response) {
        let original_bytes = serialized_len(response);
        *response = json!({
            "truncated": true,
            "original_bytes": original_bytes,
            "reason": "Code Mode response metadata exceeded the response budget",
        });
    }
}

/// Preserve the shape and completeness signal of a `fleet.map` result. Each
/// member receives an explicit `truncated` flag; values that cannot fit their
/// fair share of the response budget are replaced with a compact summary.
fn summarize_fleet_result(response: &mut Value) {
    let Some(results) = response.get_mut("result").and_then(Value::as_array_mut) else {
        return;
    };
    if results.is_empty()
        || !results.iter().all(|item| {
            item.get("name").and_then(Value::as_str).is_some()
                && item.get("ok").and_then(Value::as_bool).is_some()
        })
    {
        return;
    }

    // Leave room for the envelope, calls, logs, and JSON punctuation. This is a
    // byte budget (like the final transport cap), not an estimated token count.
    let per_instance_budget = FLEET_RESULT_BUDGET
        .checked_div(results.len())
        .unwrap_or(0)
        .max(256);

    for item in results {
        truncate_fleet_row(item, per_instance_budget);
    }
}

/// Bound one fleet row before it enters the aggregate result or QuickJS heap.
pub(crate) fn truncate_fleet_row(item: &mut Value, budget: usize) {
    if item.get("truncated").and_then(Value::as_bool) == Some(true) {
        return;
    }
    if !serialized_len_exceeds(item, budget) {
        if let Some(object) = item.as_object_mut() {
            object.insert("truncated".into(), Value::Bool(false));
        }
        return;
    }

    let Some(object) = item.as_object_mut() else {
        *item = json!({"ok": false, "truncated": true});
        return;
    };
    let name = object.get("name").cloned().unwrap_or(Value::Null);
    let ok = object.get("ok").cloned().unwrap_or(Value::Bool(false));
    let outcome = object.get("outcome").cloned().unwrap_or(Value::Null);
    let elapsed = object.get("elapsed_ms").cloned().unwrap_or(Value::Null);
    let payload = object
        .remove("value")
        .or_else(|| object.remove("error"))
        .unwrap_or(Value::Null);
    let payload_len = serialized_len(&payload);
    let payload_type = match &payload {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    };
    let item_count = match &payload {
        Value::Array(items) => Some(items.len()),
        Value::Object(fields) => Some(fields.len()),
        _ => None,
    };
    let mut summary = serde_json::Map::from_iter([
        ("type".into(), Value::String(payload_type.into())),
        ("original_bytes".into(), json!(payload_len)),
    ]);
    if let Some(count) = item_count {
        summary.insert("item_count".into(), json!(count));
    }
    *item = json!({
        "name": name,
        "ok": ok,
        "outcome": outcome,
        "value": null,
        "error": if ok == Value::Bool(false) { Value::String("upstream error exceeded the per-instance response budget".into()) } else { Value::Null },
        "truncated": true,
        "summary": summary,
        "elapsed_ms": elapsed,
    });
}

/// True iff the compact serialization of `value` is within budget.
fn within_budget(value: &Value) -> bool {
    !serialized_len_exceeds(value, RESPONSE_BUDGET)
}

fn serialized_len(value: &Value) -> usize {
    let mut writer = CountingWriter::new(usize::MAX);
    serde_json::to_writer(&mut writer, value).map_or(usize::MAX, |()| writer.bytes)
}

fn serialized_len_exceeds(value: &Value, limit: usize) -> bool {
    let mut writer = CountingWriter::new(limit);
    serde_json::to_writer(&mut writer, value).is_err() || writer.exceeded
}

struct CountingWriter {
    bytes: usize,
    limit: usize,
    exceeded: bool,
}

impl CountingWriter {
    const fn new(limit: usize) -> Self {
        Self {
            bytes: 0,
            limit,
            exceeded: false,
        }
    }
}

impl std::io::Write for CountingWriter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.bytes = self.bytes.saturating_add(buffer.len());
        if self.bytes > self.limit {
            self.exceeded = true;
            return Err(std::io::Error::other("serialization budget exceeded"));
        }
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Remove array `field` from the envelope (replacing it with `[]`) and return its
/// items. Returns empty if the field is absent or not an array.
fn take_array(response: &mut Value, field: &str) -> Vec<Value> {
    match response.get_mut(field) {
        Some(slot) if slot.is_array() => match std::mem::replace(slot, Value::Array(Vec::new())) {
            Value::Array(items) => items,
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

/// Replace `response["result"]` with a structured truncation marker, but only if
/// the marker is smaller than the original result (otherwise a small result that
/// isn't the bottleneck would needlessly grow).
fn marker_oversized_result(response: &mut Value) {
    let Some(result) = response.get("result") else {
        return;
    };
    if result.is_null() {
        return;
    }
    const NEXT_ACTION: &str = "Result exceeded the Code Mode response budget. Narrow it: \
                               request fewer fields, add limit/offset/filter params, or \
                               writeArtifact() the full payload and return a summary.";
    // Fail SAFE: an unserializable result (near-impossible for a Value) is replaced
    // with a minimal marker rather than left for the blunt transport cap.
    let Ok(serialized) = serde_json::to_string(result) else {
        response["result"] = json!({ "truncated": true, "next_action": NEXT_ACTION });
        return;
    };
    let marker = json!({
        "truncated": true,
        "original_bytes": serialized.len(),
        "original_tokens": serialized.len() / TOKEN_DIVISOR,
        "preview": utf8_prefix(&serialized, PREVIEW_BYTES),
        "next_action": NEXT_ACTION,
    });
    if serialized_len(&marker) < serialized.len() {
        response["result"] = marker;
    }
}

/// Fit the largest suffix (newest entries) of `items` into `response[field]` that
/// keeps the envelope within budget, prepending `sentinel(dropped)` when any are
/// dropped. Serialized item lengths are computed once, so probing is linear in
/// the total payload size and retained values are moved exactly once.
fn fit_newest(
    response: &mut Value,
    field: &str,
    mut items: Vec<Value>,
    sentinel: impl Fn(usize) -> Value,
) {
    let total = items.len();
    if total == 0 {
        return;
    }

    response[field] = Value::Array(Vec::new());
    let base_len = serialized_len(response).saturating_sub(2);
    let item_lengths = items.iter().map(serialized_len).collect::<Vec<_>>();
    let mut retained_bytes = 0usize;
    let mut best = 0usize;
    for keep in 0..=total {
        if keep > 0 {
            retained_bytes = retained_bytes.saturating_add(item_lengths[total - keep]);
        }
        let dropped = total - keep;
        let sentinel_len = (dropped > 0).then(|| serialized_len(&sentinel(dropped)));
        let element_count = keep + usize::from(sentinel_len.is_some());
        let array_len = 2usize
            .saturating_add(retained_bytes)
            .saturating_add(sentinel_len.unwrap_or(0))
            .saturating_add(element_count.saturating_sub(1));
        if base_len.saturating_add(array_len) <= RESPONSE_BUDGET {
            best = keep;
        }
    }

    let dropped = total - best;
    let retained = items.split_off(dropped);
    let mut out = Vec::with_capacity(best + usize::from(dropped > 0));
    if dropped > 0 {
        out.push(sentinel(dropped));
    }
    out.extend(retained);
    response[field] = Value::Array(out);
}

/// The largest char-boundary prefix of `s` that is at most `max_bytes` bytes.
fn utf8_prefix(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

#[cfg(test)]
#[path = "truncate_tests.rs"]
mod tests;
