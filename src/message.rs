extern crate libc;

use libc::{c_int, size_t, c_void};
use std::mem;

#[repr(C)]
pub struct nl_msg;
#[repr(C)]
pub struct nlmsghdr;

#[link(name="nl-3")]
extern "C" {
	// Exposed msg functions
	fn nlmsg_alloc() -> *const nl_msg;
	fn nlmsg_free(msg: *const nl_msg);
	fn nlmsg_append(msg: *const nl_msg, data: *const c_void, len: size_t, pad: c_int) -> i32;
    fn nlmsg_put(msg: *const nl_msg, pid: u32, seq: u32, mtype: c_int, payload: c_int, flags: c_int) -> *const nlmsghdr;
    fn nlmsg_datalen(nlh: *const nlmsghdr) -> i32;
    fn nlmsg_next(nlh: *const nlmsghdr, remaining: *const i32) -> *const nlmsghdr;
    fn nlmsg_inherit(nlh: *const nlmsghdr) -> *const nl_msg;
    fn nlmsg_hdr(msg: *const nl_msg) -> *const nlmsghdr;
    fn nlmsg_ok(msg: *const nl_msg) -> u32;
}

pub struct NetlinkMessage {
	ptr: *const nl_msg,
    hdr: *const nlmsghdr,
}

pub fn alloc() -> NetlinkMessage {
    let mptr = unsafe { nlmsg_alloc() };
    NetlinkMessage {
        ptr: mptr,
        hdr: unsafe { nlmsg_hdr(mptr) }
    }
}

pub fn free(msg: NetlinkMessage) {
    unsafe { nlmsg_free(msg.ptr) }
}

pub fn append<T>(msg: &mut NetlinkMessage, data: &T, len: u32, pad: i32) -> i32 {
    unsafe { 
        let vptr: *const c_void = mem::transmute(data);
        nlmsg_append(msg.ptr, vptr, len as size_t, pad as c_int) as i32 
    }
}

pub fn put(msg: &mut NetlinkMessage, pid: u32, seq: u32, mtype: i32, payload: i32, flags: i32) -> bool {

        let hdr = unsafe { nlmsg_put(msg.ptr, pid, seq, mtype as c_int, payload as c_int, flags as c_int) };

        match hdr as i32 {
            0x0 => false,
            _ => {
                true
            }
        }
}

pub fn data_len(msg: &NetlinkMessage) -> i32 {

            unsafe { nlmsg_datalen(msg.hdr) }
}

pub fn inherit(msg: &NetlinkMessage) -> NetlinkMessage {

    let mptr = unsafe { nlmsg_inherit(msg.hdr) };

    NetlinkMessage {
        ptr: mptr,
        hdr: unsafe { nlmsg_hdr(mptr) }
    }

}

pub mod expose {
    pub fn nl_msg_ptr(msg: &::message::NetlinkMessage) -> *const ::message::nl_msg {
        msg.ptr
    }

    pub fn nlmsghdr_ptr(msg: &::message::NetlinkMessage) -> *const ::message::nlmsghdr {
        msg.hdr
    }
}
