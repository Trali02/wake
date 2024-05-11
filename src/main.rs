use directories::ProjectDirs;
use local_ip_address::local_ip;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::net::UdpSocket;
use std::vec;

const CONFIG_FILENAME: &str = "MAC.config";

fn main() {
    let config_dir = ProjectDirs::from("", "", "wake").unwrap();
    let config_path = config_dir.config_dir();
    fs::create_dir_all(config_path).expect("couldn't create config dir");
    if !config_path.join(CONFIG_FILENAME).exists() {
        File::create(config_path.join(CONFIG_FILENAME)).expect("Couldn't create config file!");
    }

    let file_str = read_to_string(config_path.join(CONFIG_FILENAME)).unwrap();
    let lines = file_str.lines().collect::<Vec<&str>>();

    let mut map = HashMap::<&str, &str>::new();

    for line in lines {
        let words = line.split(':').collect::<Vec<&str>>();
        if words.len() < 2 {
            continue;
        }
        map.insert(words[0].trim(), words[1].trim());
    }

    let args: Vec<String> = env::args().collect();
    let mut mac_str = if args.len() < 2 {
        map.values()
            .next()
            .expect("No MAC address could be resolved!")
    } else {
        args[1].as_str()
    };

    if map.contains_key(mac_str) {
        mac_str = map[mac_str];
    }

    let mac = parse_mac(mac_str);

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
        let my_local_ip = local_ip().unwrap();
        let socket = UdpSocket::bind(format!("{}:0", my_local_ip))?;
        socket.set_broadcast(true)?;

        socket.send_to(&buf, "255.255.255.255:0")?;
    } // the socket is closed here
      //println!("{:?}",buf);
    Ok(())
}

fn parse_mac(mac: &str) -> Vec<u8> {
    let x = mac.split('-').collect::<Vec<&str>>();
    x.iter().map(|hex| str_to_byte(*hex)).collect::<Vec<u8>>()
}
