#[macro_use]
extern crate rsnl;

fn main() {
	let mut nls = rsnl::socket::alloc().unwrap();

    rsnl::socket::connect(&mut nls, 30);
	let buf = 0;

	rsnl::socket::send_simple(&nls, 0xfa, 0, &buf, 0);


    let mut msg = rsnl::message::alloc().unwrap();

    let code = b"Foobar\0";
    let r = rsnl::message::append(&mut msg, &code, 7, 0);
    println!("Value: {}", r);

    match rsnl::message::put(&mut msg, 1,1,3,0,0) {
        true => println!("Added header"),
        false => println!("Failed to add header")
    }

    println!("payload len: {}", rsnl::message::data_len(&msg));

    let mut nldata: rsnl::message::NetlinkData<u32> = rsnl::message::NetlinkData::new();

    let d: u32 =  4004;
    nldata.set(&d);


    let q = NlaPutU32!(&mut msg, 0, &nldata);
    println!("Put: {}", q);
    println!("payload len: {}", rsnl::message::data_len(&msg));
    let p = NlaPutU32!(&mut msg, 0, &nldata);
    println!("Put: {}", p);

    println!("payload len: {}", rsnl::message::data_len(&msg));

    let f : rsnl::message::NetlinkData<u32> =  rsnl::message::NetlinkData::with_data(&d);

    let val = f.get();

    println!("Data: {}", val.unwrap());
}
