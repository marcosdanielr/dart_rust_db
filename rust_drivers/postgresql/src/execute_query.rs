use std::ffi::{CStr, CString, c_char};

use postgres::Client;

#[unsafe(no_mangle)]
pub extern "C" fn execute_query(client_ptr: *mut Client, query: *const c_char) -> *mut c_char {
    if client_ptr.is_null() || query.is_null() {
        let error_msg = CString::new("Invalid client or query").unwrap();
        return error_msg.into_raw();
    }

    let client = unsafe { &mut *client_ptr };
    let c_str = unsafe { CStr::from_ptr(query) };
    let query_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            let error_msg = CString::new("Invalid UTF-8 in query").unwrap();
            return error_msg.into_raw();
        }
    };

    match client.query(query_str, &[]) {
        Ok(rows) => {
            let result = CString::new(format!("Found {} rows", rows.len())).unwrap();
            result.into_raw()
        }
        Err(e) => {
            let error_msg = CString::new(format!("Failed to execute query: {}", e)).unwrap();
            error_msg.into_raw()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        ffi::{CStr, CString},
    };

    use postgres::Client;

    use crate::{
        connect_to_posgresql::connect_to_postgresql, execute_query::execute_query,
        free_postgresql_connection::free_postgresql_connection,
    };

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
    fn test_execute_query_success() {
        let client_ptr = create_test_client();

        let query = CString::new("SELECT * FROM users").unwrap();
        let query_ptr = query.as_ptr();

        let result_ptr = execute_query(client_ptr, query_ptr);
        let result_str = unsafe { CStr::from_ptr(result_ptr).to_str().unwrap() };

        println!("{}", result_str);

        assert!(result_str.contains("Found"));
        assert!(result_str.contains("rows"));

        free_postgresql_connection(client_ptr);
    }
}
