use std::{net::Ipv6Addr, process::Command};

pub fn add_neighbour<S: AsRef<str>>(broadcast_iface: S, addr: Ipv6Addr) {
    let _ = Command::new("ip")
        .arg("-6")
        .arg("neigh")
        .arg("add")
        .arg("proxy")
        .arg(format!("{}", addr))
        .arg("dev")
        .arg(broadcast_iface.as_ref())
        .output();
}
