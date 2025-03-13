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
