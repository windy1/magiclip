use super::browser::AvahiMdnsBrowser;
use super::util;
use avahi_sys::{
    avahi_client_free, avahi_entry_group_add_service, avahi_entry_group_commit,
    avahi_entry_group_is_empty, avahi_entry_group_new, avahi_simple_poll_free,
    avahi_simple_poll_loop, AvahiClient, AvahiClientState, AvahiEntryGroup, AvahiEntryGroupState,
    AvahiSimplePoll,
};
use libc::{c_int, c_void};
use std::ffi::CString;
use std::ptr;

// TODO: better error reporting - missing bindings

pub struct AvahiMdnsService {
    client: *mut AvahiClient,
    poller: *mut AvahiSimplePoll,
    user_data: *mut c_void,
}

#[derive(Debug)]
pub struct UserData {
    name: CString,
    kind: CString,
    port: u16,
    group: *mut AvahiEntryGroup,
}

impl AvahiMdnsService {
    pub fn new(name: &str, kind: &str, port: u16) -> Option<Self> {
        let mut err: c_int = 0;

        let poller = util::new_poller()?;

        let user_data = Box::into_raw(Box::new(UserData {
            name: CString::new(name.to_string()).unwrap(),
            kind: CString::new(kind.to_string()).unwrap(),
            port,
            group: ptr::null_mut(),
        })) as *mut c_void;

        let client = util::new_client(poller, Some(client_callback), user_data, &mut err)?;

        match err {
            0 => Some(Self {
                client,
                poller,
                user_data,
            }),
            _ => {
                unsafe { avahi_simple_poll_free(poller) };
                None
            }
        }
    }

    pub fn start(&self) {
        unsafe { avahi_simple_poll_loop(self.poller) };
    }
}

impl Drop for AvahiMdnsService {
    fn drop(&mut self) {
        unsafe {
            avahi_client_free(self.client);
            avahi_simple_poll_free(self.poller);
            Box::from_raw(self.user_data);
        }
    }
}

extern "C" fn client_callback(
    client: *mut AvahiClient,
    state: AvahiClientState,
    userdata: *mut c_void,
) {
    match state {
        avahi_sys::AvahiClientState_AVAHI_CLIENT_S_RUNNING => {
            create_service(client, unsafe { &mut *(userdata as *mut UserData) })
        }
        _ => {}
    }
}

fn create_service(client: *mut AvahiClient, user_data: &mut UserData) {
    if user_data.group == ptr::null_mut() {
        println!("Creating group");

        user_data.group =
            unsafe { avahi_entry_group_new(client, Some(entry_group_callback), ptr::null_mut()) };

        if user_data.group == ptr::null_mut() {
            panic!("avahi_entry_group_new() failed");
        }
    }

    if unsafe { avahi_entry_group_is_empty(user_data.group) } != 0 {
        println!("Adding service");

        let ret = unsafe {
            avahi_entry_group_add_service(
                user_data.group,
                util::AVAHI_IF_UNSPEC,
                util::AVAHI_PROTO_UNSPEC,
                0,
                user_data.name.as_ptr(),
                user_data.kind.as_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                user_data.port,
            )
        };

        if ret < 0 {
            if ret == util::AVAHI_ERR_COLLISION {
                panic!("could not register service due to collision");
            }
            panic!("failed to register service");
        }

        if unsafe { avahi_entry_group_commit(user_data.group) < 0 } {
            panic!("failed to commit service");
        }
    }
}

extern "C" fn entry_group_callback(
    _group: *mut AvahiEntryGroup,
    state: AvahiEntryGroupState,
    _userdata: *mut c_void,
) {
    // TODO: handle collisions - missing binding

    match state {
        avahi_sys::AvahiEntryGroupState_AVAHI_ENTRY_GROUP_ESTABLISHED => {
            println!("GROUP_ESTABLISHED");
            AvahiMdnsBrowser::new("_magiclip._tcp").unwrap().start();
        }
        _ => {}
    }
}
