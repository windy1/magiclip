extern crate avahi_sys;
extern crate clipboard;
extern crate libc;

use clipboard::{ClipboardContext, ClipboardProvider};
use magiclip::service::AvahiMdnsService;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{:?}", ctx.get_contents());

    AvahiMdnsService::new("test", "_magiclip._tcp", 42069)
        .unwrap()
        .start();
}
