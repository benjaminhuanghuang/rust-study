fn main() {
    let ip = String::from("127.0.0.1");
    let ip_slice = &ip[3..];     // borrow of `ip` occurs here

    use_ip(ip);    // move out of `ip` occurs here
    dbg!(ip_slice);      // borrow later used here. Comment out this line, error disappear
}

fn use_ip(ip:String) {
    println!("{}", ip);
}