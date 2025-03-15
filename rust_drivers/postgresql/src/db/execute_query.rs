use postgres::Client;

use std::ffi::{CStr, CString, c_char};
use std::ptr;

use crate::enums::operation_type::OperationType;
use crate::utils::identify_operation_type::identify_operation_type;

use super::query::queries::{modification_query, select_query};
use super::query::query_result::QueryResult;

#[unsafe(no_mangle)]
pub extern "C" fn execute_query(client_ptr: *mut Client, query: *const c_char) -> *mut QueryResult {
    if client_ptr.is_null() || query.is_null() {
        return create_error_result("Invalid client or query");
    }

    let client = unsafe { &mut *client_ptr };
    let c_str = unsafe { CStr::from_ptr(query) };
    let query_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return create_error_result("Invalid UTF-8 in query"),
    };

    let operation_type = identify_operation_type(query_str);

    match operation_type {
        OperationType::Select => select_query(client, query_str),
        OperationType::Delete | OperationType::Update | OperationType::Insert => {
            modification_query(client, query_str)
        }
        OperationType::Unknown => Box::into_raw(Box::new(QueryResult {
            success: false,
            affected_rows: 0,
            result_ptr: ptr::null_mut(),
            result_len: 0,
            error_ptr: "Unknown operation type".as_ptr() as *mut c_char,
        })),
    }
}

fn create_error_result(error_msg: &str) -> *mut QueryResult {
    let c_error = CString::new(error_msg).unwrap();

    let result = Box::new(QueryResult {
        success: false,
        affected_rows: 0,
        result_ptr: ptr::null_mut(),
        result_len: 0,
        error_ptr: c_error.into_raw(),
    });

    Box::into_raw(result)
}

#[unsafe(no_mangle)]
pub extern "C" fn free_query_result(result_ptr: *mut QueryResult) {
    if !result_ptr.is_null() {
        unsafe {
            let result = Box::from_raw(result_ptr);

            if !result.result_ptr.is_null() {
                let _ =
                    Vec::from_raw_parts(result.result_ptr, result.result_len, result.result_len);
            }

            if !result.error_ptr.is_null() {
                let _ = CString::from_raw(result.error_ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{env, ffi::CString};

    use crate::db::{
        connect_to_postgresql::connect_to_postgresql,
        free_postgresql_connection::free_postgresql_connection,
    };

    use super::*;

    fn create_test_client() -> *mut Client {
        let connection_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");
        let c_connection_url = CString::new(connection_url).unwrap();
        let ptr_connection_url = c_connection_url.as_ptr();

        let client_ptr = connect_to_postgresql(ptr_connection_url);

        if client_ptr.is_null() {
            panic!("Failed to connect to PostgreSQL in test");
        }

        client_ptr
    }

    #[test]
    fn test_select_query() {
        let client_ptr = create_test_client();

        let query = CString::new("SELECT id, first_name FROM users LIMIT 2").unwrap();
        let query_ptr = query.as_ptr();

        let result_ptr = execute_query(client_ptr, query_ptr);
        let result = unsafe { &*result_ptr };

        assert!(result.success);
    }

    #[test]
    fn test_update_query() {
        let client_ptr = create_test_client();
        let query = CString::new(
            "UPDATE users SET first_name = 'updated' WHERE id = 'bc26ec22-67e3-4c99-a576-635f6c44d3f3'",
        )
        .unwrap();
        let query_ptr = query.as_ptr();

        let result_ptr = execute_query(client_ptr, query_ptr);
        let result = unsafe { &*result_ptr };

        assert!(result.success);

        free_query_result(result_ptr);
        free_postgresql_connection(client_ptr);
    }

    #[test]
    fn test_delete_query() {
        let client_ptr = create_test_client();

        let query: CString =
            CString::new("DELETE FROM users WHERE id = 'bc26ec22-67e3-4c99-a576-635f6c44d3f3'")
                .unwrap();
        let query_ptr = query.as_ptr();

        let result_ptr = execute_query(client_ptr, query_ptr);
        let result = unsafe { &*result_ptr };

        assert!(result.success);

        free_query_result(result_ptr);
        free_postgresql_connection(client_ptr);
    }
}
