use default_net::{get_default_gateway, interface, Gateway};
use serde::Serialize;
use std::env;
use std::net::IpAddr;

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
    macaddress: String,
    ipaddress: String,
    gateway: String,
    hostname: String,
}

impl NetworkData {
    fn new() -> Self {
        let gw = get_default_gateway().unwrap_or(Gateway::new());
        let ip: IpAddr = interface::get_local_ipaddr().unwrap_or(IpAddr::from([0, 0, 0, 0]));
        let hn = gethostname::gethostname()
            .into_string()
            .unwrap_or("Invalid_hostname".to_string());
        Self {
            macaddress: gw.mac_addr.to_string(),
            ipaddress: ip.to_string(),
            gateway: gw.ip_addr.to_string(),
            hostname: hn,
        }
    }
}

#[tokio::main]
async fn main() {
    let nwd = NetworkData::new();
    match get_default_gateway() {
        Ok(gateway) => {
            println!("Default Gateway found");
            println!("{}", gateway.mac_addr.to_string());
        }
        Err(e) => {
            println!("{}", e);
        }
    };

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
