extern crate libc;

use libc::{c_int};


#[repr(C)]
pub struct nl_cb;

#[link(name="nl-3")]
extern "C" {
	// Exposed socket functions
	fn nl_cb_alloc(kind: c_int) -> *const nl_cb;
}

pub enum Kind {
    Default,
    Verbose,
    Debug,
    Custom,
    __Max
}

pub enum Type {
    Valid,
    Finish,
    Overrun,
    Skipped,
    Ack,
    MsgIn,
    MsgOut,
    SeqCheck,
    SendAck,
    DumpIntr,
    __Max
}

pub enum Action {
    Ok,
    Skip,
    Stop
}

pub struct NetlinkCallback {
    ptr: *const nl_cb
}

pub fn alloc(kind: Kind) -> Option<NetlinkCallback> {

    let cbptr = unsafe { nl_cb_alloc(kind as c_int) };
    
    match cbptr as i32 {
        0x0 => None,
        _  => Some(NetlinkCallback{ptr: cbptr})
    }
}
