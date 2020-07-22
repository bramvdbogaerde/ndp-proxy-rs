/// Module responsible for parsing command line arguments
mod cli;

/// Modules responsible for communicating with the Linux iproute2
/// utility
mod iproute2;

use cli::Command;
use etherparse::{Ethernet2HeaderSlice, Ipv6HeaderSlice};
use pcap::{Capture, Device, Packet};
use std::io;

const IPV6_ETHER_TYPE: u16 = 0x86DD;

fn get_device_by_name<S: AsRef<str>>(name: S) -> Option<Device> {
    let devices = Device::list();
    match devices {
        Ok(devices) => devices
            .into_iter()
            .filter(|device| device.name == name.as_ref())
            .next(),
        Err(_) => None,
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn receive_packet<S: AsRef<str>>(packet: &Packet, prefix: &cli::Ipv6Prefix, broadcast_iface: S) -> Result<()> {
    let ethernet_header = Ethernet2HeaderSlice::from_slice(packet.data)?;

    // we are only interested in ipv6 packets
    if ethernet_header.ether_type() == IPV6_ETHER_TYPE {
        let ipv6_header =
            Ipv6HeaderSlice::from_slice(&packet.data[ethernet_header.slice().len()..])?;

        // we are only interested in packets coming from a specific prefix
        if prefix.matches(ipv6_header.source_addr()) {
            iproute2::add_neighbour(broadcast_iface, ipv6_header.source_addr());
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let command = Command::parse();
    let device = get_device_by_name(command.iface_name).expect("invalid interface name");
    let mut capture = Capture::from_device(device)
        .unwrap()
        .promisc(true)
        .timeout(1)
        .open()
        .unwrap();

    println!("Listening for packets...");

    let prefix = command.prefix;
    while let Ok(packet) = capture.next() {
        match receive_packet(&packet, &prefix, &command.broadcast_iface) {
            Err(_) => println!("packet dropped"),
            Ok(_) => (),
        }
    }

    println!("Packet capture finished");
    Ok(())
}
