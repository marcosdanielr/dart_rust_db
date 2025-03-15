use std::ffi::CStr;

use super::query_result::QueryResult;

pub fn _print_query_result(result_ptr: *mut QueryResult) {
    if result_ptr.is_null() {
        println!("Error: result_ptr is null");
        return;
    }

    let result = unsafe { &*result_ptr };

    if !result.success {
        let error_message = unsafe { CStr::from_ptr(result.error_ptr).to_string_lossy() };
        println!("Query failed: {}", error_message);
        return;
    }

    if result.result_len > 0 {
        let binary_data =
            unsafe { std::slice::from_raw_parts(result.result_ptr, result.result_len) };

        let data_str = String::from_utf8_lossy(binary_data);
        let lines: Vec<&str> = data_str.lines().collect();

        for line in lines {
            let fields: Vec<&str> = line.split(',').collect();
            println!("{:?}", fields);
        }
    } else {
        println!("No results returned");
    }
}
