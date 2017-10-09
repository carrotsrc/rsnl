use libc::{c_int, size_t, c_void};
use std::mem;
use std::marker::PhantomData;

#[repr(C)]
pub struct nl_msg {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct nlmsghdr {
    _unused: [u8; 0],
}

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
    fn nlmsg_data(msg: *const nlmsghdr) -> *const c_void;
}

pub struct NetlinkMessage {
	ptr: *const nl_msg,
    hdr: *const nlmsghdr,
}

pub struct NetlinkData <T> {
    ptr: Option<*const c_void>,
    phantom: PhantomData<T>
}

impl <T> NetlinkData <T> {

    pub fn new() -> NetlinkData<T> {
        NetlinkData {
            ptr: None,
            phantom: PhantomData
        }
    }

    pub fn with_data<D>(data: &D) -> NetlinkData<T> {
        NetlinkData {
            ptr: Some(unsafe{ mem::transmute(data) }),
            phantom: PhantomData
        }
    }

    pub fn with_vptr(data: *const c_void) -> NetlinkData<T> {
        NetlinkData {
            ptr: Some(data),
            phantom: PhantomData
        }
    }

    pub fn get(&self) -> Option<&T> {
        match self.ptr {
        None => None,
        Some(vptr) => {
            Some( unsafe { mem::transmute(vptr) } )
            }
        }
    }

    pub fn set(&mut self, data: &T) {
        match self.ptr {

        None => {
            let p: *const c_void =  unsafe{ mem::transmute(data) };
            self.ptr = Some( p );
            },

         _   => return

        }
    }

    pub fn from_vptr(&mut self, data: *const c_void) {
        match self.ptr {
            None    => self.ptr = Some(data),
            _       =>  return
        }
    }

    pub fn to_vptr(&self) -> Option<*const c_void> {
        self.ptr
    }
}

pub fn contain(ptr: *const nl_msg) -> Option<NetlinkMessage> {
    match ptr as isize {
        0x0 => None,
        _   => Some ( 
            NetlinkMessage {
                ptr: ptr,
                hdr: unsafe { nlmsg_hdr(ptr) }
            })
    }
}

pub fn alloc() -> Option<NetlinkMessage> {
    let mptr = unsafe { nlmsg_alloc() };
    contain(mptr)
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

pub fn data<T>(msg: &NetlinkMessage, container: &mut NetlinkData<T>) {

    unsafe {
        let vptr = nlmsg_data(msg.hdr);
        container.from_vptr(vptr);
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
