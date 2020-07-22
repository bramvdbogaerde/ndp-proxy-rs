use clap::Clap;
use std::net::Ipv6Addr;

#[derive(Clap)]
pub struct CliCommand {
    pub iface_name: String,
    pub broadcast_iface: String,
    #[clap(short)]
    pub prefix: String,
}   

pub struct Command {
    pub iface_name: String,
    pub broadcast_iface: String,
    pub prefix: Ipv6Prefix,
}

pub struct Ipv6Prefix {
    addr: Ipv6Addr,
    length: usize
}

impl Ipv6Prefix {
    /// Check if the given prefix matches the given ipv6 address 
    pub fn matches(&self, addr: Ipv6Addr) -> bool {
        let my_octets = &self.addr.octets()[0..self.length/8];
        let other_octets = &addr.octets()[0..self.length/8];
        my_octets == other_octets
    }
}

impl From<CliCommand> for Command {
    fn from(cli_command: CliCommand) -> Command {
        let prefix = cli_command.parse_prefix().unwrap();
        Command {
            iface_name: cli_command.iface_name,
            broadcast_iface: cli_command.broadcast_iface,
            prefix,
        }
    }
}

impl CliCommand {
    /// Parses the prefix given from the command
    /// The prefix should be formatted as a:b:c:d::/LENGTH
    pub fn parse_prefix(&self) -> Result<Ipv6Prefix, Box<dyn std::error::Error>> {
        let mut parts = self.prefix.split("/");
        let prefix = parts.next().ok_or("could not detect prefix in ipv6 prefix")?;
        let prefix_length = parts.next().ok_or("could not detect prefix length")?;
        Ok(Ipv6Prefix {
            addr: prefix.parse()?,
            length: prefix_length.parse()?,
        })
    }
}

impl Command {
    pub fn parse() -> Command {
        let cli_command = <CliCommand as Clap>::parse();
        cli_command.into()
    }
}
