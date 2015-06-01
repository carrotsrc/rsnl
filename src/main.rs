extern crate rsnl;

//use rsnl::{socket};


fn main() {
	let nls = rsnl::socket::new();
	nls.connect(rsnl::NetlinkProtocol::zu);
	let buf = 0;

	nls.send_simple(0xfa, 0, &buf, 0);

    let msg = rsnl::msg::new();
    let code = b"Foobar\0";
    let r = msg.append(&code, 7, 0);
    println!("Value: {}", r);
}
