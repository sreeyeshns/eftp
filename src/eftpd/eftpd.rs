use clap::Parser;
use std::net::{IpAddr, Ipv4Addr};
use log::info;

/// Command line options
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run as a daemon
    #[arg(short, long)]
    daemon: bool,

    /// IP address to bind
    #[arg(short, long, required(false))]
    ip_addr: String,
    
    /// TCP port to bind
    #[arg(short, long, required(false))]
    port: u16,
}

struct EftpServer {
    is_daemon: bool,
    ip_addr: IpAddr,
    port: u16
}

impl EftpServer {
    fn new() -> EftpServer{
        EftpServer {
            is_daemon: false,
            ip_addr: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port:0,
        }
    }
    fn set(self, args: &Args) {
        self.is_daemon = args.daemon;
        self.ip_addr = IpAddr::V4((Ipv4Addr::from(args.ip_addr)));
        self.port = args.port;
    }
}

fn main() {
    // Initialize logging
    env_logger::init();

    info!("Starting eftpd!");

    // Parse commandline options

    let mut options = Options::new();
    parse_options(&mut options);

}


fn parse_options(options: &mut Options) {
    let args = Args::parse();
    
    for _ in 0..args. {
        println!("Hello {}!", args.name)
    }
}
