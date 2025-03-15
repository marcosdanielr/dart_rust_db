use postgres::{Row, types::Type};

pub fn serialize_rows_to_binary(rows: &[Row]) -> Vec<u8> {
    let mut output = Vec::new();

    let row_count = rows.len() as u32;
    output.extend_from_slice(&row_count.to_le_bytes());

    if rows.is_empty() {
        return output;
    }

    let columns = rows[0].columns();
    let column_count = columns.len() as u32;
    output.extend_from_slice(&column_count.to_le_bytes());

    for column in columns {
        let name = column.name().as_bytes();
        let name_len = name.len() as u32;
        output.extend_from_slice(&name_len.to_le_bytes());
        output.extend_from_slice(name);

        let type_oid = column.type_().oid() as u32;
        output.extend_from_slice(&type_oid.to_le_bytes());
    }

    for row in rows {
        for (i, column) in row.columns().iter().enumerate() {
            let is_null = row
                .try_get::<_, Option<String>>(i)
                .map_or(true, |v| v.is_none());
            let null_flag = if is_null { 1u8 } else { 0u8 };
            output.push(null_flag);

            if is_null {
                continue;
            }

            match *column.type_() {
                Type::BOOL => {
                    if let Ok(val) = row.try_get::<_, bool>(i) {
                        let byte = if val { 1u8 } else { 0u8 };
                        output.push(byte);
                    }
                }
                Type::INT2 => {
                    if let Ok(val) = row.try_get::<_, i16>(i) {
                        output.extend_from_slice(&val.to_le_bytes());
                    }
                }
                Type::INT4 => {
                    if let Ok(val) = row.try_get::<_, i32>(i) {
                        output.extend_from_slice(&val.to_le_bytes());
                    }
                }
                Type::INT8 => {
                    if let Ok(val) = row.try_get::<_, i64>(i) {
                        output.extend_from_slice(&val.to_le_bytes());
                    }
                }
                Type::FLOAT4 => {
                    if let Ok(val) = row.try_get::<_, f32>(i) {
                        output.extend_from_slice(&val.to_le_bytes());
                    }
                }
                Type::FLOAT8 => {
                    if let Ok(val) = row.try_get::<_, f64>(i) {
                        output.extend_from_slice(&val.to_le_bytes());
                    }
                }
                _ => {
                    if let Ok(val) = row.try_get::<_, String>(i) {
                        let bytes = val.as_bytes();
                        let len = bytes.len() as u32;
                        output.extend_from_slice(&len.to_le_bytes());
                        output.extend_from_slice(bytes);
                    }
                }
            }
        }
    }

    output
}
