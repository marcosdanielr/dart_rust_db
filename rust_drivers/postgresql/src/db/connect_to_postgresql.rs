use postgres::{Client, NoTls};
use std::ffi::CStr;
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

#[cfg(test)]
mod tests {
    use std::{env, ffi::CString};

    use crate::db::{
        connect_to_postgresql::connect_to_postgresql,
        free_postgresql_connection::free_postgresql_connection,
    };

    #[test]
    fn test_connection_success() {
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
    fn test_failed_connection() {
        let invalid_url = "invalid_url";

        let c_invalid_url = CString::new(invalid_url).expect("Failed to create CString");
        let ptr_invalid_url = c_invalid_url.as_ptr();

        let result = connect_to_postgresql(ptr_invalid_url);

        assert!(
            result.is_null(),
            "The connection should fail and return a null pointer"
        );
    }
}
