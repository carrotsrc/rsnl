extern crate libc;

use libc::{c_int, size_t, c_void};
use std::mem;

#[repr(C)]
struct nl_msg;
#[repr(C)]
struct nlmsghdr;

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
    hdr: Option<*const nlmsghdr>,
}

pub fn alloc() -> NetlinkMessage {
    NetlinkMessage {
        ptr: unsafe { nlmsg_alloc() },
        hdr: None
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
    unsafe {
        let hdr = nlmsg_put(msg.ptr, pid, seq, mtype as c_int, payload as c_int, flags as c_int);

        match hdr as i32 {
            0x0 => false,
            _ => {
                msg.hdr = Some(hdr);
                true
            }
        }
    }
}

pub fn data_len(msg: &NetlinkMessage) -> i32 {

    match msg.hdr {
        None => 0,
        Some(hdr) => {
            unsafe { nlmsg_datalen(hdr) }
        }
    }
}

pub fn inherit(msg: &NetlinkMessage) -> NetlinkMessage {
    if msg.hdr == None {
        return alloc();
    }

    let mut m = NetlinkMessage {
        ptr: unsafe { nlmsg_inherit(msg.hdr.unwrap()) },
        hdr: None
    };

    m.hdr = Some( unsafe {nlmsg_hdr(msg.ptr)} );
    m
}
