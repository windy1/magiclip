use crate::mdns::err;
use avahi_sys::{
    avahi_entry_group_add_service, avahi_entry_group_commit, AvahiEntryGroup, AvahiIfIndex,
    AvahiProtocol, AvahiPublishFlags,
};
use libc::c_char;
use std::ffi::CStr;

#[derive(Builder)]
pub struct AvahiServiceFactory {
    group: *mut AvahiEntryGroup,
    interface: AvahiIfIndex,
    protocol: AvahiProtocol,
    flags: AvahiPublishFlags,
    name: *const c_char,
    kind: *const c_char,
    domain: *const c_char,
    host: *const c_char,
    port: u16,
}

impl AvahiServiceFactory {
    pub fn create_service(&self) -> Result<(), String> {
        unsafe {
            println!("name = {:?}", CStr::from_ptr(self.name).to_str());
        };

        println!("group = {:?}", self.group);
        println!("kind = {:?}", self.kind);
        println!("port = {:?}", self.port);

        println!("DEBUG1");
        let err = unsafe {
            avahi_entry_group_add_service(
                self.group,
                self.interface,
                self.protocol,
                self.flags,
                self.name,
                self.kind,
                self.domain,
                self.host,
                self.port,
            )
        };

        println!("DEBUG2");

        if err < 0 {
            return Err(format!(
                "could not register service: {}",
                err::get_error(err)
            ));
        }

        if unsafe { avahi_entry_group_commit(self.group) } < 0 {
            Err("could not commit service".to_string())
        } else {
            Ok(())
        }
    }

    pub fn builder() -> AvahiServiceFactoryBuilder {
        AvahiServiceFactoryBuilder::default()
    }
}
