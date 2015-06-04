extern crate libc;

use libc::{c_int, c_void};
use std::mem;


#[repr(C)]
pub struct nl_sock;
#[repr(C)]
pub struct nl_cb;

#[link(name="nl-3")]
extern "C" {
	// Exposed socket functions
	fn nl_socket_alloc() -> *const nl_sock;
	fn nl_socket_free(socket: *const nl_sock);
	fn nl_socket_get_fd(socket: *const nl_sock) -> i32;
	fn nl_socket_set_buffer_size(socket: *const nl_sock, rxbuf: c_int, txbuf: c_int) -> i32;

	fn nl_socket_set_cb(socket: *const nl_sock, cb: *const nl_cb);
	fn nl_socket_get_cb(socket: *const nl_sock) -> nl_cb;

	fn nl_socket_set_local_port(socket: *const nl_sock, port: u32);
	fn nl_socket_get_local_port(socket: *const nl_sock) -> u32;

	fn nl_connect(socket: *const nl_sock, protocol: u32) -> i32;
	fn nl_close(socket: *const nl_sock);

	// Exposed socket transceivers
	fn nl_send_simple(socket: *const nl_sock, msg_type: c_int, flags: c_int, buf: *const c_void, size: c_int) -> i32;
}

pub struct NetlinkSocket {
    pub ptr: *const nl_sock,
}


pub fn alloc() -> NetlinkSocket {
    NetlinkSocket {
        ptr: unsafe { nl_socket_alloc() },
    }
}

pub fn free(sock: NetlinkSocket) {
    unsafe { nl_socket_free(sock.ptr); }
}

pub fn set_buffer_size(sock: &mut NetlinkSocket, rxbuf: i32, txbuf: i32) -> i32 {
    unsafe { nl_socket_set_buffer_size(sock.ptr, rxbuf, txbuf) }
}

pub fn connect(sock: &mut NetlinkSocket, protocol: u32) -> i32 {
    unsafe { nl_connect(sock.ptr, protocol) }
}

pub fn close(sock: &mut NetlinkSocket) {
    unsafe{ nl_close(sock.ptr) }
}

pub fn get_local_port(sock: &NetlinkSocket) -> u32 {
    unsafe { nl_socket_get_local_port(sock.ptr) }
}

pub fn set_local_port(sock: &mut NetlinkSocket, port: u32) {
    unsafe { nl_socket_set_local_port(sock.ptr, port) }
}

pub fn get_fd(sock: &NetlinkSocket) -> i32 {
    unsafe { nl_socket_get_fd(sock.ptr) }
}

pub fn send_simple<T>(sock: &NetlinkSocket, msg_type: i32, flags: i32, buf: &T, size: i32) -> i32 {
    unsafe { 
        let vptr: *const c_void = mem::transmute(buf);
        nl_send_simple(sock.ptr, msg_type, flags, vptr, size) as i32 
    }
}

pub mod expose {
    pub fn nl_sock_ptr(sock: &mut ::socket::NetlinkSocket) -> *const ::socket::nl_sock {
        sock.ptr
    }

}
