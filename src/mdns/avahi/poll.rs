use avahi_sys::{avahi_simple_poll_new, AvahiSimplePoll};
use std::ptr;

pub fn new_poller() -> Result<*mut AvahiSimplePoll, String> {
    let poller = unsafe { avahi_simple_poll_new() };
    if poller == ptr::null_mut() {
        Err("could not initialize Avahi simple poll".to_string())
    } else {
        Ok(poller)
    }
}
