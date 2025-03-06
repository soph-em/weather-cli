use local_ip_address::local_ip;
use std::net::IpAddr;

fn get_ip() -> IpAddr {
    let my_local_ip = local_ip().unwrap();
    return my_local_ip;
}
