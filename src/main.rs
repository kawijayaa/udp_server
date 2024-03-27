use std::str::from_utf8;

use etherparse::{EtherType, IpNumber, Ipv4Slice, UdpSlice};
use tun_tap::{Iface, Mode};
fn main() {
    let tun = Iface::new("", Mode::Tun).expect("Cannot create tunnel!");
    println!("[+] Created tunnel with name {}", tun.name());

    loop {
        let mut recv_buffer = [0u8; 1504];
        let recv_len = tun.recv(&mut recv_buffer[..]).unwrap();

        let ether_protocol = u16::from_be_bytes([recv_buffer[2], recv_buffer[3]]);
        let ether_packet = &recv_buffer[4..recv_len];

        if ether_protocol == EtherType::IPV4.0 {
            let ipv4_packet =
                Ipv4Slice::from_slice(ether_packet).expect("Cannot parse IPv4 packet!");

            if ipv4_packet.header().protocol() == IpNumber::UDP {
                let udp_packet = UdpSlice::from_slice(ipv4_packet.payload().payload)
                    .expect("Cannot parse UDP packet!");
                let recv_data = from_utf8(udp_packet.payload()).unwrap();
                println!(
                    "Received from {}:{} to port {} : {}",
                    ipv4_packet.header().source_addr(),
                    udp_packet.source_port(),
                    udp_packet.destination_port(),
                    recv_data
                );
            }
        }
    }
}
