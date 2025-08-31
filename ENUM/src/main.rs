#[derive(Debug)]
enum IpAddrkind {
    v4,
    v6,
}

struct IpAddress {
    address: String,
    kind: IpAddrkind,
}

fn main() {

    let google_Add = IpAddress{
        address: String::from("1.2.3.4"),
        kind: IpAddrkind::v4,
    };

    route(google_Add);
}


fn route(ip: IpAddress){
    println!("Routing Request to IP {} of Kind {:?} ", ip.address, ip.kind);
}