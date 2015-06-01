extern crate rsnl;

//use rsnl::{socket};


fn main() {
	let nls = rsnl::socket::new();
	nls.connect(rsnl::NetlinkProtocol::zu);
	let buf = 0;

	nls.send_simple(0xfa, 0, &buf, 0);
}
