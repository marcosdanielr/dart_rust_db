use postgres::{Client, NoTls};
use std::ffi::CStr;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn connect_to_postgresql(connection_url: *const c_char) -> bool {
    if connection_url.is_null() {
        eprintln!("Connection URL is null");
        return false;
    }

    let c_str = unsafe { CStr::from_ptr(connection_url) };
    let conn_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Invalid UTF-8 in connection string");
            return false;
        }
    };

    match Client::connect(conn_str, NoTls) {
        Ok(_) => {
            println!("Connected successfully");
            true
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::connect_to_postgresql;
    use std::{env, ffi::CString};

    #[test]
    fn test_postgresql_connection_success() {
        let connection_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");

        let c_connection_url = CString::new(connection_url).expect("Failed to create CString");

        let ptr_connection_url = c_connection_url.as_ptr();

        let success = connect_to_postgresql(ptr_connection_url);

        assert!(success, "Failed to connect to the PostgreSQL");
    }

    #[test]
    fn test_failed_postgresql_connection() {
        let invalid_url = "invalid_url";

        let c_invalid_url = CString::new(invalid_url).expect("Failed to create CString");
        let ptr_invalid_url = c_invalid_url.as_ptr();

        let success = connect_to_postgresql(ptr_invalid_url);

        assert!(!success, "The connection should fail with an invalid URL");
    }
}
