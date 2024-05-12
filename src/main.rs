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

const ARG_MAC_TARGET: &str = "-m";
const ARG_MAC_TARGET_VERBOSE: &str = "--mac";
const ARG_LOOKUP_TARGET: &str = "-l";
const ARG_LOOKUP_TARGET_VERBOSE: &str = "--lookup";
const ARG_HELP: &str = "-h";
const ARG_HELP_VERBOSE: &str = "--help";

const HELP_MESSAGE: &str = "-m, --mac    : takes the mac address as argument, 
               the mac address format is FF-FF-FF-FF-FF-FF
-l, --lookup : takes the lookup name and uses the mac 
               address specified in the config file";

#[derive(Debug)]
enum ProgArgs {
    MacTarget(String),
    LookupTarget(String),
    Help,
    None,
}

fn main() {
    let prog_args = get_args();

    // ---------- SETUP --------------

    let config_dir = ProjectDirs::from("", "", "wake").unwrap();
    let config_path = config_dir.config_dir();
    fs::create_dir_all(config_path).expect("Couldn't create config dir!");
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

    // -------- LOGIC ------------------

    for arg in prog_args {
        match arg {
            ProgArgs::MacTarget(mac_addr) => {
                let addr = mac_addr.to_uppercase();
                match send_wake(addr.as_str()) {
                    Ok(_) => {}
                    Err(_) => println!("Failed to send package to Mac address \"{}\"", addr),
                }
            }
            ProgArgs::LookupTarget(lookup_name) => {
                if !map.contains_key(lookup_name.as_str()) {
                    println!("Failed to resolve lookup target \"{}\"", lookup_name);
                    continue;
                }
                let addr = map[lookup_name.as_str()];
                match send_wake(addr) {
                    Ok(_) => {}
                    Err(_) => println!("Failed to send package to Mac address \"{}\"", addr),
                }
            }
            ProgArgs::None => {}
            ProgArgs::Help => println!("{}", HELP_MESSAGE),
        }
    }
}

fn str_to_byte(hex: &str) -> u8 {
    u8::from_str_radix(hex, 16).expect("String could not be resolved to a hex format!")
}

fn send_wake(addr: &str) -> std::io::Result<()> {
    let addr_raw = parse_mac(addr);

    let mut buf: Vec<u8> = Vec::with_capacity(102);
    let mut p1: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    buf.append(&mut p1);
    let mac_addr_vec: Vec<u8> = addr_raw;
    for _ in 0..16 {
        buf.append(&mut mac_addr_vec.clone());
    }
    {
        let my_local_ip = local_ip().unwrap();
        let socket = UdpSocket::bind(format!("{}:0", my_local_ip))?;
        socket.set_broadcast(true)?;

        socket.send_to(&buf, "255.255.255.255:0")?;
    }
    println!("Sent package to MAC address \"{}\"", addr.to_uppercase());
    Ok(())
}

fn parse_mac(mac: &str) -> Vec<u8> {
    let x = mac.split('-').collect::<Vec<&str>>();
    x.iter().map(|hex| str_to_byte(*hex)).collect::<Vec<u8>>()
}

fn get_args() -> Vec<ProgArgs> {
    let args_raw: Vec<String> = env::args().collect();
    let mut args: Vec<ProgArgs> = Vec::new();
    let mut args_iter = args_raw.iter();
    args_iter.next();
    while let Some(arg) = args_iter.next() {
        match arg.to_lowercase().as_str() {
            ARG_MAC_TARGET | ARG_MAC_TARGET_VERBOSE => args.push(match args_iter.next() {
                Some(mac_addr) => ProgArgs::MacTarget(mac_addr.clone()),
                None => ProgArgs::None,
            }),
            ARG_LOOKUP_TARGET | ARG_LOOKUP_TARGET_VERBOSE => args.push(match args_iter.next() {
                Some(lookup_name) => ProgArgs::LookupTarget(lookup_name.clone()),
                None => ProgArgs::None,
            }),
            ARG_HELP | ARG_HELP_VERBOSE => args.push(ProgArgs::Help),
            _ => args.push(ProgArgs::None),
        }
    }
    return args;
}
