use crate::enums::operation_type::OperationType;

pub fn identify_operation_type(query: &str) -> OperationType {
    let query_lower = query.trim().to_lowercase();

    match query_lower.split_ascii_whitespace().next() {
        Some("select") => OperationType::Select,
        Some("insert") => OperationType::Insert,
        Some("update") => OperationType::Update,
        Some("delete") => OperationType::Delete,
        _ => OperationType::Unknown,
    }
}
