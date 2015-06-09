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
    let val = nldata.get();

    println!("Data: {}", val.unwrap());

    let p = rsnl::attribute::put(&mut msg, rsnl::attribute::Type::U32, std::mem::size_of::<u32>() as u32, &nldata);
    println!("Put: {}", p);


}
