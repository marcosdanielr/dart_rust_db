use postgres::{Client, NoTls};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn connect_to_postgresql(connection_url: *const c_char) -> *mut Client {
    if connection_url.is_null() {
        return ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(connection_url) };
    let conn_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    match Client::connect(conn_str, NoTls) {
        Ok(client) => Box::into_raw(Box::new(client)),
        Err(_) => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_postgresql_connection(client: *mut Client) {
    if !client.is_null() {
        unsafe {
            let _ = Box::from_raw(client);
        }
    }
}

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
    use postgres::Client;

    use crate::{connect_to_postgresql, execute_query, free_postgresql_connection};
    use std::{
        env,
        ffi::{CStr, CString},
    };

    #[test]
    fn test_postgresql_connection_success() {
        let connection_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");

        let c_connection_url = CString::new(connection_url).expect("Failed to create CString");

        let ptr_connection_url = c_connection_url.as_ptr();

        let result = connect_to_postgresql(ptr_connection_url);

        assert!(
            !result.is_null(),
            "The connection should succeed and return a non-null pointer"
        );

        free_postgresql_connection(result);
    }

    #[test]
    fn test_failed_postgresql_connection() {
        let invalid_url = "invalid_url";

        let c_invalid_url = CString::new(invalid_url).expect("Failed to create CString");
        let ptr_invalid_url = c_invalid_url.as_ptr();

        let result = connect_to_postgresql(ptr_invalid_url);

        assert!(
            result.is_null(),
            "The connection should fail and return a null pointer"
        );
    }

    fn create_test_client() -> *mut Client {
        let connection_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");
        let c_connection_url = CString::new(connection_url).unwrap();
        let ptr_connection_url = c_connection_url.as_ptr();

        connect_to_postgresql(ptr_connection_url)
    }

    #[test]
    fn test_execute_query_success() {
        let client_ptr = create_test_client();

        let query = CString::new("SELECT * FROM users").unwrap();
        let query_ptr = query.as_ptr();

        let result_ptr = execute_query(client_ptr, query_ptr);

        let result_str = unsafe { CStr::from_ptr(result_ptr).to_str().unwrap() };

        print!("{}", result_str);

        assert!(result_str.contains("Found"));
        assert!(result_str.contains("rows"));
    }
}
