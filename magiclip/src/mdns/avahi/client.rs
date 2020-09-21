use super::err;
use super::poll::ManagedAvahiSimplePoll;
use crate::builder::BuilderDelegate;
use avahi_sys::{
    avahi_client_free, avahi_client_new, avahi_simple_poll_get, AvahiClient, AvahiClientCallback,
    AvahiClientFlags,
};
use libc::{c_int, c_void};
use std::ptr;

pub struct ManagedAvahiClient {
    pub(super) client: *mut AvahiClient,
}

#[derive(Builder)]
pub struct ManagedAvahiClientParams<'a> {
    poll: &'a ManagedAvahiSimplePoll,
    flags: AvahiClientFlags,
    callback: AvahiClientCallback,
    userdata: *mut c_void,
}

impl ManagedAvahiClient {
    pub fn new(
        ManagedAvahiClientParams {
            poll,
            flags,
            callback,
            userdata,
        }: ManagedAvahiClientParams,
    ) -> Result<Self, String> {
        let mut err: c_int = 0;

        let client = unsafe {
            avahi_client_new(
                avahi_simple_poll_get(poll.poll),
                flags,
                callback,
                userdata,
                &mut err,
            )
        };

        if client == ptr::null_mut() {
            return Err("could not initialize AvahiClient".to_string());
        }

        match err {
            0 => Ok(Self { client }),
            _ => Err(format!(
                "could not initialize AvahiClient: {}",
                err::get_error(err)
            )),
        }
    }
}

impl Drop for ManagedAvahiClient {
    fn drop(&mut self) {
        unsafe { avahi_client_free(self.client) };
    }
}

impl<'a> BuilderDelegate<ManagedAvahiClientParamsBuilder<'a>> for ManagedAvahiClientParams<'a> {}
