use std::net::Ipv4Addr;

fn ping<A>(address: A) -> std::io::Result<bool>
where
    A: Into<Ipv4Addr>,
{
    println!("{}", address.into());
    Ok(true)
}

fn main() {
    let mut _x = ping(Ipv4Addr::new(1, 2, 3, 4));
    _x = ping([1, 2, 3, 4]);
    _x = ping(0x1234_u32);
}
