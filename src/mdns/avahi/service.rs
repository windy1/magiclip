use super::client::{ManagedAvahiClient, ManagedAvahiClientParams};
use super::constants;
use super::entry_group::{AddServiceParams, ManagedAvahiEntryGroup, ManagedAvahiEntryGroupParams};
use super::poll::ManagedAvahiSimplePoll;
use crate::util::BuilderDelegate;
use avahi_sys::{
    AvahiClient, AvahiClientFlags, AvahiClientState, AvahiEntryGroup, AvahiEntryGroupState,
};
use libc::c_void;
use std::ffi::CString;
use std::fmt::{self, Formatter};
use std::ptr;

pub struct MdnsService {
    poll: Option<ManagedAvahiSimplePoll>,
    context: *mut AvahiServiceContext,
}

impl MdnsService {
    pub fn new(kind: &str, port: u16) -> Self {
        Self {
            poll: None,
            context: Box::into_raw(Box::new(AvahiServiceContext::new(kind, port))),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe { (*self.context).name = Some(CString::new(name.to_string()).unwrap()) };
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("MdnsService#start()\n");

        self.poll = Some(ManagedAvahiSimplePoll::new()?);

        Some(ManagedAvahiClient::new(
            ManagedAvahiClientParams::builder()
                .poll(self.poll.as_ref().unwrap())
                .flags(AvahiClientFlags(0))
                .callback(Some(client_callback))
                .userdata(self.context as *mut c_void)
                .build()?,
        )?);

        self.poll.as_ref().unwrap().start_loop()
    }
}

impl Drop for MdnsService {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.context) };
    }
}

struct AvahiServiceContext {
    name: Option<CString>,
    kind: CString,
    port: u16,
    group: Option<ManagedAvahiEntryGroup>,
}

impl AvahiServiceContext {
    fn new(kind: &str, port: u16) -> Self {
        Self {
            name: None,
            kind: CString::new(kind).unwrap(),
            port,
            group: None,
        }
    }
}

impl fmt::Debug for AvahiServiceContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AvahiServiceContext")
            .field("name", &self.name)
            .field("kind", &self.kind)
            .field("port", &self.port)
            .field("has_group", &self.group.is_some())
            .finish()
    }
}

unsafe extern "C" fn client_callback(
    client: *mut AvahiClient,
    state: AvahiClientState,
    userdata: *mut c_void,
) {
    println!("client_callback()");

    let context = &mut *(userdata as *mut AvahiServiceContext);

    println!("context = {:?}", context);

    match state {
        avahi_sys::AvahiClientState_AVAHI_CLIENT_S_RUNNING => create_service(client, context),
        avahi_sys::AvahiClientState_AVAHI_CLIENT_FAILURE => panic!("client failure"),
        avahi_sys::AvahiClientState_AVAHI_CLIENT_S_REGISTERING => {
            if let Some(g) = &mut context.group {
                g.reset();
            }
            println!("group reset");
        }
        _ => {}
    };

    println!();
}

fn create_service(client: *mut AvahiClient, context: &mut AvahiServiceContext) {
    println!("create_service()");

    if context.group.is_none() {
        println!("creating group\n");

        context.group = Some(
            ManagedAvahiEntryGroup::new(
                ManagedAvahiEntryGroupParams::builder()
                    .client(client)
                    .callback(Some(entry_group_callback))
                    .userdata(ptr::null_mut())
                    .build()
                    .unwrap(),
            )
            .unwrap(),
        );
    }

    println!("context = {:?}", context);

    let group = context.group.as_mut().unwrap();

    if group.is_empty() {
        println!("adding service");

        group
            .add_service(
                AddServiceParams::builder()
                    .interface(constants::AVAHI_IF_UNSPEC)
                    .protocol(constants::AVAHI_PROTO_UNSPEC)
                    .flags(0)
                    .name(context.name.as_ref().unwrap().as_ptr())
                    .kind(context.kind.as_ptr())
                    .domain(ptr::null_mut())
                    .host(ptr::null_mut())
                    .port(context.port)
                    .build()
                    .unwrap(),
            )
            .unwrap();
    }

    println!();
}

extern "C" fn entry_group_callback(
    _group: *mut AvahiEntryGroup,
    state: AvahiEntryGroupState,
    _userdata: *mut c_void,
) {
    println!("entry_group_callback()");

    match state {
        avahi_sys::AvahiEntryGroupState_AVAHI_ENTRY_GROUP_ESTABLISHED => {
            println!("GROUP_ESTABLISHED");
        }
        _ => {}
    };

    println!();
}
