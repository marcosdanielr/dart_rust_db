use std::ffi::CString;
use std::ptr;

use crate::db::query::query_result::QueryResult;

pub fn create_error_result(error_msg: &str) -> *mut QueryResult {
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
