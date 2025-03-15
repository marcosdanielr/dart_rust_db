use postgres::Client;

#[unsafe(no_mangle)]
pub extern "C" fn free_postgresql_connection(client: *mut Client) {
    if !client.is_null() {
        unsafe {
            let _ = Box::from_raw(client);
        }
    }
}
