#![feature(libc)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]


pub mod message;
pub mod socket;

/**
 * Abstraction of libnl core functionality
 */
extern crate libc;
#[link(name="nl-3")]

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

