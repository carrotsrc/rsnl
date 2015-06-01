#![feature(libc)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate libc;
use libc::{c_int, size_t, c_void};
use std::mem;

/**
 * Abstraction of libnl core functionality
 */
#[link(name="nl-3")]
extern "C" {
	// Exposed socket functions
	fn nl_socket_alloc() -> *const nl_sock;
	fn nl_socket_free(socket: *const nl_sock);
	fn nl_socket_get_fd(socket: *const nl_sock) -> i32;
	fn nl_socket_set_buffer_size(socket: *const nl_sock, rxbuf: c_int, txbuf: c_int) -> i32;
	
	fn nl_socket_set_local_port(socket: *const nl_sock, port: u32);
	fn nl_socket_get_local_port(socket: *const nl_sock) -> u32;

	fn nl_connect(socket: *const nl_sock, protocol: u32) -> i32;
	fn nl_close(socket: *const nl_sock);

	fn nl_socket_set_cb(socket: *const nl_sock, cb: *const nl_cb);
	fn nl_socket_get_cb(socket: *const nl_sock) -> nl_cb;

	// Exposed socket transceiver
	fn nl_send_simple(socket: *const nl_sock, msg_type: c_int, flags: c_int, buf: *const u8, size: c_int) -> i32;

	// Exposed msg functions
	fn nlmsg_alloc() -> *const nl_msg;
	fn nlmsg_free(msg: *const nl_msg);
	fn nlmsg_append(msg: *const nl_msg, data: *const c_void, len: size_t, pad: c_int) -> i32;
    fn nlmsg_put(msg: *const nl_msg, pid: u32, seq: u32, mtype: c_int, payload: c_int, flags: c_int) -> *const nlmsghdr;


}

// exposed structures - these are wrapped
#[repr(C)]
struct nl_sock;
#[repr(C)]
struct nl_msg;
#[repr(C)]
struct nl_cb;
#[repr(C)]
struct nlmsghdr;

// RSNL datatypes wrapping the libnl data structures
pub struct socket {
	ptr: *const nl_sock
}


/* library version of nlmsghdr
 * the name is altered to differentiate
 * between the native libnl and rsnl
 */
pub struct rsnl_msghdr {
	nlmsg_len: u32,
	nlmsg_type: u16,
	nlmsg_flags: u16,
	nlmsg_seq: u32,
	nlmsg_pid: u32
}

pub struct msg {
	ptr: *const nl_msg,
}

pub enum NetlinkProtocol {
	route,
	unused,
	usersock,
	firewall,
	sock_diag,
	nflog,
	xfrm,
	selinux,
	iscsi,
	audit,
	fib_lookup,
	connector,
	netfilter,
	ip6_fw,
	dnrtmsg,
	kobject_uevent,
	DMEVENTS,
	scsitransport,
	ecryptfs,
	rdma,
	crypto,
	zu= 30
}


impl socket {
	pub fn new() -> socket {
		unsafe {
			let nlptr = nl_socket_alloc();
			socket {
				ptr: nlptr
			}
		}
	}

	pub fn free(&self) {
		unsafe {
			nl_socket_free(self.ptr)
		}
	}

	pub fn set_buffer_size(&self, rxbuf: c_int, txbuf: c_int) -> i32 {
		unsafe {
			nl_socket_set_buffer_size(self.ptr, rxbuf, txbuf)
		}
	}

	pub fn connect(&self, protocol: NetlinkProtocol) -> i32 {
		unsafe { nl_connect(self.ptr, protocol as u32) }
	}

	pub fn close(&self) {
		unsafe{ nl_close(self.ptr) }
	}

	pub fn get_fd(&self) -> i32 {
		unsafe { nl_socket_get_fd(self.ptr) }
	}

	pub fn get_local_port(&self) -> u32 {
		unsafe { nl_socket_get_local_port(self.ptr) }
	}

	pub fn set_local_port(&self, port: u32) {
		unsafe { nl_socket_set_local_port(self.ptr, port) }
	}

	pub fn send_simple(&self, msg_type: c_int, flags: c_int, buf: *const u8, size: c_int) -> i32 {
		unsafe { nl_send_simple(self.ptr, msg_type, flags, buf, size) as i32 }
	}
}

impl msg {

	pub fn new() -> msg {

	unsafe {
		let nlmsg = nlmsg_alloc();
		msg { 
			ptr: nlmsg
		}
	}
	}


	pub fn free(&self) {
		unsafe{ nlmsg_free(self.ptr); }
	}

    pub fn append<T>(&self, data: &T, len: u32, pad: i32) -> i32 {
        unsafe { 
            let vptr: *const c_void = mem::transmute(data);
            nlmsg_append(self.ptr, vptr, len as size_t, pad as c_int) as i32 
        }
    }

    pub fn put(&self, pid: u32, seq: u32, mtype: i32, payload: i32, flags: i32) {
        unsafe {
            nlmsg_put(self.ptr, pid, seq, mtype as c_int, payload as c_int, flags as c_int);
        }
    }
}
