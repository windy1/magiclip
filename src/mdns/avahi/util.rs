use avahi_sys::{
    avahi_client_new, avahi_simple_poll_get, avahi_simple_poll_new, AvahiClient,
    AvahiClientCallback, AvahiClientFlags, AvahiSimplePoll,
};
use libc::{c_int, c_void};
use std::ptr;

pub const AVAHI_IF_UNSPEC: i32 = -1;
pub const AVAHI_PROTO_UNSPEC: i32 = -1;
pub const AVAHI_ERR_COLLISION: i32 = -8;
pub const AVAHI_ADDRESS_STR_MAX: usize = 40;

pub fn new_poller() -> Option<*mut AvahiSimplePoll> {
    let poller = unsafe { avahi_simple_poll_new() };
    if poller == ptr::null_mut() {
        None
    } else {
        Some(poller)
    }
}

pub fn new_client(
    poller: *mut AvahiSimplePoll,
    callback: AvahiClientCallback,
    user_data: *mut c_void,
    err: *mut c_int,
) -> Option<*mut AvahiClient> {
    let client = unsafe {
        avahi_client_new(
            avahi_simple_poll_get(poller),
            AvahiClientFlags(0),
            callback,
            user_data,
            err,
        )
    };

    if client == ptr::null_mut() {
        None
    } else {
        Some(client)
    }
}
