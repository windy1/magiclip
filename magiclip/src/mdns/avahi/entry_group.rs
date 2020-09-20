use super::err;
use crate::util::BuilderDelegate;
use avahi_sys::{
    avahi_entry_group_add_service, avahi_entry_group_commit, avahi_entry_group_free,
    avahi_entry_group_is_empty, avahi_entry_group_new, avahi_entry_group_reset, AvahiClient,
    AvahiEntryGroup, AvahiEntryGroupCallback, AvahiIfIndex, AvahiProtocol, AvahiPublishFlags,
};
use libc::{c_char, c_void};
use std::ptr;

#[derive(Debug)]
pub struct ManagedAvahiEntryGroup {
    group: *mut AvahiEntryGroup,
}

impl ManagedAvahiEntryGroup {
    pub fn new(
        ManagedAvahiEntryGroupParams {
            client,
            callback,
            userdata,
        }: ManagedAvahiEntryGroupParams,
    ) -> Result<Self, String> {
        let group = unsafe { avahi_entry_group_new(client, callback, userdata) };
        if group == ptr::null_mut() {
            Err("could not initialize AvahiEntryGroup".to_string())
        } else {
            Ok(Self { group })
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { avahi_entry_group_is_empty(self.group) != 0 }
    }

    pub fn add_service(
        &mut self,
        AddServiceParams {
            interface,
            protocol,
            flags,
            name,
            kind,
            domain,
            host,
            port,
        }: AddServiceParams,
    ) -> Result<(), String> {
        use std::ffi::CStr;

        unsafe {
            println!("add_service()");
            println!("group = {:?}", self.group);
            println!("interface = {:?}", interface);
            println!("protocol = {:?}", protocol);
            println!("flags = {:?}", flags);
            println!("name = {:?}", CStr::from_ptr(name));
            println!("kind = {:?}", CStr::from_ptr(kind));
            println!("domain = {:?}", domain);
            println!("host = {:?}", host);
            println!("port = {:?}", port);
        }

        let err = unsafe {
            avahi_entry_group_add_service(
                self.group,
                interface,
                protocol,
                flags,
                name,
                kind,
                domain,
                host,
                port,
                ptr::null_mut() as *const c_char, // null terminated txt record list
            )
        };

        println!("err = {:?}", err);

        if err < 0 {
            return Err(format!(
                "could not register service: `{}`",
                err::get_error(err)
            ));
        }

        let err = unsafe { avahi_entry_group_commit(self.group) };

        if err < 0 {
            Err(format!(
                "could not commit service: `{}`",
                err::get_error(err)
            ))
        } else {
            Ok(())
        }
    }

    pub fn reset(&mut self) {
        unsafe { avahi_entry_group_reset(self.group) };
    }
}

impl Drop for ManagedAvahiEntryGroup {
    fn drop(&mut self) {
        unsafe { avahi_entry_group_free(self.group) };
    }
}

#[derive(Builder)]
pub struct ManagedAvahiEntryGroupParams {
    client: *mut AvahiClient,
    callback: AvahiEntryGroupCallback,
    userdata: *mut c_void,
}

impl<'a> BuilderDelegate<ManagedAvahiEntryGroupParamsBuilder> for ManagedAvahiEntryGroupParams {}

#[derive(Builder)]
pub struct AddServiceParams {
    interface: AvahiIfIndex,
    protocol: AvahiProtocol,
    flags: AvahiPublishFlags,
    name: *const c_char,
    kind: *const c_char,
    domain: *const c_char,
    host: *const c_char,
    port: u16,
}

impl BuilderDelegate<AddServiceParamsBuilder> for AddServiceParams {}
