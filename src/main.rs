// use default_net::{get_default_gateway, interface, Gateway};
use serde::Serialize;
use std::env;
use default_net::interface::MacAddr;
// use std::net::IpAddr;

fn usage() {
    println!(
        r#"Usage:\
        gwmacpush <url to post to>.\

    Posts 
    struct NetworkData {{
        macaddress: String,
        ipaddress: String,
        gateway: String,
        hostname: String,
    }} as JSON
    to URL"#
    );
    std::process::exit(0);
}

#[derive(Serialize)]
struct NetworkData {
    macaddresses: Vec<String>,
    hostname: String,
}
fn empty_octett(m:&Option<MacAddr>) -> bool {
    match m {
        Some(mac) => mac.octets() != [0,0,0,0,0,0],
        None => false
    }
}

impl NetworkData {
    fn new() -> Self {
        let hn = gethostname::gethostname()
            .into_string()
            .unwrap_or("Invalid_hostname".to_string());

        let filtered_interfaces = default_net::get_interfaces().into_iter()
                         .filter(|p| p.gateway.is_some())   // Filter all without gateways
                         .filter(|p| empty_octett(&p.mac_addr)) // filter all with empty mac addresses
                         .collect::<Vec< default_net::Interface>>();
        let interfaces: Vec<String> = filtered_interfaces.into_iter()
                         .map(|p| p.mac_addr.expect("checked before").address())
                         .collect::<Vec<String>>();

        Self {
            macaddresses: interfaces,
            hostname: hn,
        }
    }
}

#[tokio::main]
async fn main() {
    let nwd = NetworkData::new();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }
    let url = &args[1];
    println!("In file {}", url);
    let client = reqwest::Client::new();
    match client.post(url).json(&nwd).send().await {
        Ok(data) => println!("{}", data.status()),
        Err(e) => println!("{}", e),
    };
}
