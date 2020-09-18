use super::err;
use avahi_sys::{
    avahi_client_new, avahi_simple_poll_get, AvahiClient, AvahiClientCallback, AvahiClientFlags,
    AvahiSimplePoll,
};
use libc::{c_int, c_void};
use std::convert::TryInto;
use std::ptr;

#[derive(Builder)]
pub struct AvahiClientParams {
    poller: *mut AvahiSimplePoll,
    callback: AvahiClientCallback,
    context: *mut c_void,
}

impl AvahiClientParams {
    pub fn builder() -> AvahiClientParamsBuilder {
        AvahiClientParamsBuilder::default()
    }
}

impl TryInto<*mut AvahiClient> for AvahiClientParams {
    type Error = String;

    fn try_into(self) -> Result<*mut AvahiClient, String> {
        let mut err: c_int = 0;

        let client = unsafe {
            avahi_client_new(
                avahi_simple_poll_get(self.poller), // poll_api
                AvahiClientFlags(0),                // flags
                self.callback,                      // callback
                self.context,                       // userdata
                &mut err,                           // error
            )
        };

        if client == ptr::null_mut() {
            return Err("could not initialize AvahiClient".to_string());
        }

        match err {
            0 => Ok(client),
            _ => Err(format!(
                "could not initialize AvahiClient: `{}`",
                err::get_error(err)
            )),
        }
    }
}
