use postgres::Client;
use std::ptr;

use crate::ffi::{
    allocate_binary_result::allocate_binary_result, create_error_result::create_error_result,
};

use super::{query_result::QueryResult, serialize_rows_to_binary::serialize_rows_to_binary};

pub fn select_query(client: &mut Client, query: &str) -> *mut QueryResult {
    match client.query(query, &[]) {
        Ok(rows) => {
            let binary_data = serialize_rows_to_binary(&rows);

            let result = Box::new(QueryResult {
                success: true,
                affected_rows: rows.len() as i32,
                result_ptr: allocate_binary_result(&binary_data),
                result_len: binary_data.len(),
                error_ptr: ptr::null_mut(),
            });

            Box::into_raw(result)
        }
        Err(e) => create_error_result(&e.to_string()),
    }
}

pub fn modification_query(client: &mut Client, query: &str) -> *mut QueryResult {
    match client.execute(query, &[]) {
        Ok(affected) => {
            let result = Box::new(QueryResult {
                success: true,
                affected_rows: affected as i32,
                result_ptr: ptr::null_mut(),
                result_len: 0,
                error_ptr: ptr::null_mut(),
            });

            Box::into_raw(result)
        }
        Err(e) => create_error_result(&e.to_string()),
    }
}
