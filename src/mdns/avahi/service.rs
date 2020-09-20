use super::client::AvahiClientParams;
use super::constants;
use super::poll;
use avahi_sys::{
    avahi_client_free, avahi_entry_group_add_service, avahi_entry_group_commit,
    avahi_entry_group_free, avahi_entry_group_is_empty, avahi_entry_group_new,
    avahi_entry_group_reset, avahi_simple_poll_free, avahi_simple_poll_loop, AvahiClient,
    AvahiClientState, AvahiEntryGroup, AvahiEntryGroupState, AvahiSimplePoll,
};
use libc::c_void;
use std::convert::TryInto;
use std::ffi::CString;
use std::ptr;

pub struct MdnsService {
    client: *mut AvahiClient,
    poller: *mut AvahiSimplePoll,
    context: *mut AvahiServiceContext,
}

struct AvahiServiceContext {
    name: Option<CString>,
    kind: CString,
    port: u16,
    group: *mut AvahiEntryGroup,
}

impl MdnsService {
    pub fn new(kind: &str, port: u16) -> Self {
        Self {
            client: ptr::null_mut(),
            poller: ptr::null_mut(),
            context: Box::into_raw(Box::new(AvahiServiceContext {
                name: None,
                kind: CString::new(kind.to_string()).unwrap(),
                port,
                group: ptr::null_mut(),
            })),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe { (*self.context).name = Some(CString::new(name.to_string()).unwrap()) };
    }

    pub fn start(&mut self) -> Result<(), String> {
        unsafe {
            if let None = (*self.context).name {
                return Err("service name required when using Avahi".to_string());
            }
        };

        self.poller = poll::new_poller()?;

        self.client = AvahiClientParams::builder()
            .poller(self.poller)
            .callback(Some(client_callback))
            .context(self.context as *mut c_void)
            .build()?
            .try_into()?;

        unsafe { avahi_simple_poll_loop(self.poller) };

        Ok(())
    }
}

impl Drop for MdnsService {
    fn drop(&mut self) {
        unsafe {
            if self.client != ptr::null_mut() {
                avahi_client_free(self.client);
            }

            if self.poller != ptr::null_mut() {
                avahi_simple_poll_free(self.poller);
            }

            Box::from_raw(self.context);
        }
    }
}

impl Drop for AvahiServiceContext {
    fn drop(&mut self) {
        unsafe {
            if self.group != ptr::null_mut() {
                avahi_entry_group_free(self.group);
            }
        }
    }
}

extern "C" fn client_callback(
    client: *mut AvahiClient,
    state: AvahiClientState,
    userdata: *mut c_void,
) {
    let context = unsafe { &mut *(userdata as *mut AvahiServiceContext) };

    match state {
        avahi_sys::AvahiClientState_AVAHI_CLIENT_S_RUNNING => create_service(client, context),
        avahi_sys::AvahiClientState_AVAHI_CLIENT_FAILURE => panic!("client failure"),
        avahi_sys::AvahiClientState_AVAHI_CLIENT_S_REGISTERING => {
            if userdata != ptr::null_mut() && context.group != ptr::null_mut() {
                unsafe { avahi_entry_group_reset(context.group) };
            }
        }
        _ => {}
    }
}

fn create_service(client: *mut AvahiClient, context: &mut AvahiServiceContext) {
    if context.group == ptr::null_mut() {
        println!("Creating group");

        context.group =
            unsafe { avahi_entry_group_new(client, Some(entry_group_callback), ptr::null_mut()) };

        if context.group == ptr::null_mut() {
            panic!("avahi_entry_group_new() failed");
        }
    }

    if unsafe { avahi_entry_group_is_empty(context.group) } != 0 {
        println!("Adding service");

        println!("name = {:?}", context.name);
        println!("group = {:?}", context.group);
        println!("kind = {:?}", context.kind);
        println!("port = {:?}", context.port);

        let ret = unsafe {
            avahi_entry_group_add_service(
                context.group,
                constants::AVAHI_IF_UNSPEC,
                constants::AVAHI_PROTO_UNSPEC,
                0,
                context.name.as_ref().unwrap().as_ptr(),
                context.kind.as_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                context.port,
            )
        };

        println!("Service added");

        if ret < 0 {
            if ret == constants::AVAHI_ERR_COLLISION {
                panic!("could not register service due to collision");
            } else {
                panic!("failed to register service");
            }
        }

        if unsafe { avahi_entry_group_commit(context.group) < 0 } {
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
        }
        _ => {}
    }
}
