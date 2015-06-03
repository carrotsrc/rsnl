extern crate rsnl;

//use rsnl::{socket};


fn main() {
	let mut nls = rsnl::sock_alloc();
    rsnl::connect(&mut nls, 30);
	let buf = 0;

	rsnl::send_simple(&nls, 0xfa, 0, &buf, 0);

    let mut msg = rsnl::msg_alloc();
    let code = b"Foobar\0";
    let r = rsnl::msg_append(&mut msg, &code, 7, 0);
    println!("Value: {}", r);

    match rsnl::msg_put(&mut msg, 1,1,3,0,0) {
        true => println!("Added header"),
        false => println!("Failed to add header")
    }

    println!("payload len: {}", rsnl::msg_data_len(&msg));
}
