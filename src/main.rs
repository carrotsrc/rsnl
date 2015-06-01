extern crate rsnl;

//use rsnl::{socket};


fn main() {
	let nls = rsnl::socket::new();
	nls.connect(30);
	let buf = 0;

	nls.send_simple(0xfa, 0, &buf, 0);

    let mut msg = rsnl::msg::new();
    let code = b"Foobar\0";
    let r = msg.append(&code, 7, 0);
    println!("Value: {}", r);

    match msg.put(1,1,3,0,0) {
        true => println!("Added header"),
        false => println!("Failed to add header")
    }

    println!("payload len: {}", msg.data_len());
}
