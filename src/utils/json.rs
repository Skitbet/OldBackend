use serde_json::Value;

/// Recursively merges JSON value `b` into `a`.
///
/// - If both `a` and `b` are JSON objects, merges their keys recursively.
/// - Otherwise, `a` is replaced by a clone of `b`.
pub fn json_merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(a_map), Value::Object(b_map)) => {
            for (key, b_value) in b_map {
                // For each key in b's object, recursively merge into a's entry or insert null if missing.
                json_merge(a_map.entry(key).or_insert(Value::Null), b_value);
            }
        }
        // If not both objects, replace `a` with `b`.
        (a_slot, b_value) => {
            *a_slot = b_value.clone();
        }
    }
}
