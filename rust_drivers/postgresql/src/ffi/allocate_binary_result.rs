pub fn allocate_binary_result(data: &[u8]) -> *mut u8 {
    let mut buffer = Vec::with_capacity(data.len());
    buffer.extend_from_slice(data);
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}
