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

	// Exposed socket transceivers
	fn nl_send_simple(socket: *const nl_sock, msg_type: c_int, flags: c_int, buf: *const c_void, size: c_int) -> i32;

	// Exposed msg functions
	fn nlmsg_alloc() -> *const nl_msg;
	fn nlmsg_free(msg: *const nl_msg);
	fn nlmsg_append(msg: *const nl_msg, data: *const c_void, len: size_t, pad: c_int) -> i32;
    fn nlmsg_put(msg: *const nl_msg, pid: u32, seq: u32, mtype: c_int, payload: c_int, flags: c_int) -> *const nlmsghdr;
    fn nlmsg_datalen(nlh: *const nlmsghdr) -> i32;
    fn nlmsg_next(nlh: *const nlmsghdr, remaining: *const i32) -> *const nlmsghdr;


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
}

pub struct NetlinkSocket {
    ptr: *const nl_sock,
}

pub struct NetlinkMessage {
	ptr: *const nl_msg,
    hdr: Option<*const nlmsghdr>,
}


pub fn sock_alloc() -> NetlinkSocket {
    NetlinkSocket {
        ptr: unsafe { nl_socket_alloc() },
    }
}

pub fn sock_free(sock: NetlinkSocket) {
    unsafe { nl_socket_free(sock.ptr); }
}

pub fn sock_set_buffer_size(sock: &mut NetlinkSocket, rxbuf: i32, txbuf: i32) -> i32 {
    unsafe { nl_socket_set_buffer_size(sock.ptr, rxbuf, txbuf) }
}

pub fn connect(sock: &mut NetlinkSocket, protocol: u32) -> i32 {
    unsafe { nl_connect(sock.ptr, protocol) }
}

pub fn close(sock: &mut NetlinkSocket) {
    unsafe{ nl_close(sock.ptr) }
}

pub fn sock_get_local_port(sock: &NetlinkSocket) -> u32 {
    unsafe { nl_socket_get_local_port(sock.ptr) }
}

pub fn sock_set_local_port(sock: &mut NetlinkSocket, port: u32) {
    unsafe { nl_socket_set_local_port(sock.ptr, port) }
}

pub fn sock_get_fd(sock: &NetlinkSocket) -> i32 {
    unsafe { nl_socket_get_fd(sock.ptr) }
}

pub fn send_simple<T>(sock: &NetlinkSocket, msg_type: i32, flags: i32, buf: &T, size: i32) -> i32 {
    unsafe { 
        let vptr: *const c_void = mem::transmute(buf);
        nl_send_simple(sock.ptr, msg_type, flags, vptr, size) as i32 
    }
}

pub fn msg_alloc() -> NetlinkMessage {
    NetlinkMessage {
        ptr: unsafe { nlmsg_alloc() },
        hdr: None
    }
}

pub fn msg_free(msg: NetlinkMessage) {
    unsafe { nlmsg_free(msg.ptr) }
}

pub fn msg_append<T>(msg: &mut NetlinkMessage, data: &T, len: u32, pad: i32) -> i32 {
    unsafe { 
        let vptr: *const c_void = mem::transmute(data);
        nlmsg_append(msg.ptr, vptr, len as size_t, pad as c_int) as i32 
    }
}

pub fn msg_put(msg: &mut NetlinkMessage, pid: u32, seq: u32, mtype: i32, payload: i32, flags: i32) -> bool {
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

pub fn msg_data_len(msg: &NetlinkMessage) -> i32 {

    match msg.hdr {
        None => 0,
        Some(hdr) => {
            unsafe { nlmsg_datalen(hdr) }
        }
    }
}

