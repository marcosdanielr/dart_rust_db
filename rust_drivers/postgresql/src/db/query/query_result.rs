use libc::c_char;

#[repr(C)]
pub struct QueryResult {
    pub success: bool,
    pub affected_rows: i32,
    pub result_ptr: *mut u8,
    pub result_len: usize,
    pub error_ptr: *mut c_char,
}
