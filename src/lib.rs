#[allow(dead_code)]
#[allow(non_camel_case_types)] 

/**
 * Abstraction of libnl core functionality
 */
#[link(name="nl-3")]
extern "C" {
	// Exposed socket functions
	fn nl_socket_alloc() -> *const nl_sock;
	fn nl_socket_free(socket: *const nl_sock);
	fn nl_socket_get_fd(socket: *const nl_sock) -> i32;
	fn nl_socket_set_buffer_size(socket: *const nl_sock, rxbuf: int, txbuf: int) -> i32;
	
	fn nl_socket_set_local_port(socket: *const nl_sock, port: u32);
	fn nl_socket_get_local_port(socket: *const nl_sock);

	fn nl_connect(socket: *const nl_sock, protocol: u32) -> i32;
	fn nl_close(socket: *const nl_sock);

	fn nl_socket_set_cb(socket: *const nl_sock, cb: *const nl_cb);
	fn nl_socket_get_cb(socket: *const nl_sock) -> nl_cb;

	// Exposed msg functions
	fn nlmsg_alloc() -> *const nl_msg;
	fn nlmsg_free(msg: *const nl_msg);
}

// exposed structures - these are wrapped
struct nl_sock;
struct nl_msg;
struct nl_cb;

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
	ptr: *const nl_msg
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
	crypto
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

	pub fn set_buffer_size(&self, rxbuf: int, txbuf: int) -> i32 {
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
}
