use libc::{c_int, c_void};
use std::mem;

#[repr(C)]
pub struct nl_sock {
    _unused: [u8; 0],
}

#[link(name="nl-3")]
extern "C" {
	// Exposed socket functions
	fn nl_socket_alloc() -> *const nl_sock;
	fn nl_socket_free(socket: *const nl_sock);
	fn nl_socket_get_fd(socket: *const nl_sock) -> i32;
	fn nl_socket_set_buffer_size(socket: *const nl_sock, rxbuf: c_int, txbuf: c_int) -> i32;

	fn nl_socket_set_cb(socket: *const nl_sock, cb: *const ::callback::nl_cb);
	fn nl_socket_get_cb(socket: *const nl_sock) -> ::callback::nl_cb;

	fn nl_socket_set_local_port(socket: *const nl_sock, port: u32);
	fn nl_socket_get_local_port(socket: *const nl_sock) -> u32;

	fn nl_connect(socket: *const nl_sock, protocol: u32) -> i32;
	fn nl_close(socket: *const nl_sock);

	// Exposed socket transceivers
	fn nl_send_simple(socket: *const nl_sock, msg_type: c_int, flags: c_int, buf: *const c_void, size: c_int) -> i32;
    fn nl_sendto(socket: *const nl_sock, buf: *const c_void, size: c_int) -> i32;
    fn nl_send(socket: *const nl_sock, msg: *const ::message::nl_msg) -> i32;
    fn nl_send_auto(socket: *const nl_sock, msg: *const ::message::nl_msg) -> i32;
    fn nl_sendmsg(socket: *const nl_sock, msg: *const ::message::nl_msg, hdr: *const ::message::nlmsghdr) -> i32;
}

pub struct NetlinkSocket {
    pub ptr: *const nl_sock,
}


pub fn alloc() -> Option<NetlinkSocket> {
    let ptr = unsafe { nl_socket_alloc() };

    match ptr as isize {
    0x0 => None,
    _ => Some (NetlinkSocket { ptr: ptr })
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

pub fn sendto<T>(sock: &NetlinkSocket, buf: &T, size: u32) -> i32 {
    unsafe {
        let vptr: *const c_void = mem::transmute(buf);
        nl_sendto(sock.ptr, vptr, size as c_int)
    }
}

pub fn send(sock: &NetlinkSocket, mut msg: ::message::NetlinkMessage) -> i32 {
    unsafe { nl_send(sock.ptr, ::message::expose::nl_msg_ptr(&mut msg)) }
}

pub fn send_auto(sock: &NetlinkSocket, mut msg: ::message::NetlinkMessage) -> i32 {
    unsafe { nl_send_auto(sock.ptr, ::message::expose::nl_msg_ptr(&mut msg)) }
}

pub fn sendmsg(sock: &NetlinkSocket, mut msg: ::message::NetlinkMessage) -> i32 {
    let hdr = ::message::expose::nlmsghdr_ptr(&mut msg);

    unsafe { nl_sendmsg(sock.ptr, ::message::expose::nl_msg_ptr(&mut msg), hdr) }
}

pub fn recvmsg(sock: &NetlinkSocket, fptr: fn()->i32) -> i32 {
    4
}

pub mod expose {
    pub fn nl_sock_ptr(sock: &::socket::NetlinkSocket) -> *const ::socket::nl_sock {
        sock.ptr
    }

}
