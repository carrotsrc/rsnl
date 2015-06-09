extern crate libc;

use libc::{c_int, c_void};
use ::message::{NetlinkMessage, NetlinkData, nl_msg};


#[link(name="nl-3")]
extern "C" {
    fn nla_put(msg: *const nl_msg, attrtype: c_int, datalen: c_int, data: *const c_void) -> i32;
}


pub enum Type {
    Unspec,
    U8,
    U16,
    U32,
    U64,
    String,
    Flag,
    Msecs,
    Nested,
    __Max
}

pub fn put<T>(msg: &mut NetlinkMessage, atype: Type, len: u32, data: &NetlinkData<T>) -> i32 {
    let vptr = match data.to_vptr() {
        None => return -1,
        Some(ptr) => ptr
    };
    unsafe{ nla_put(::message::expose::nl_msg_ptr(msg), atype as c_int, len as c_int, vptr) }
}




