use std::env;
use std::net::UdpSocket;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("No receiving mac adress was given");
        return;
    }
    let mac_str = &args[1];
    let x = mac_str.split('-').collect::<Vec<&str>>();
    let mac = x.iter().map(|hex| str_to_byte(*hex)).collect::<Vec<u8>>();

    println!("Sending packet to MAC: {}", mac_str);
    send_wake(mac.try_into().unwrap()).expect("Package wasn't able to be sent 🙁");
}

fn str_to_byte(hex: &str) -> u8 {
    u8::from_str_radix(hex, 16).expect("String could not be resolved!")
}

fn send_wake(mac_addr: [u8; 6]) -> std::io::Result<()> {
    let mut buf: Vec<u8> = Vec::with_capacity(102);
    let mut p1: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    buf.append(&mut p1);
    let mac_addr_vec: Vec<u8> = mac_addr.to_vec();
    for _ in 0..16 {
        buf.append(&mut mac_addr_vec.clone());
    }
    {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;

        socket.send_to(&buf, "255.255.255.255:0")?;
    } // the socket is closed here
    //println!("{:?}",buf);
    Ok(())
}
