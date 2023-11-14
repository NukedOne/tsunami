use crate::net::{create_recv_sock, IP_HDR_LEN};
use anyhow::{bail, Result};
use pnet::packet::tcp::TcpPacket;
use std::time::Duration;
use tokio::time::timeout;

pub async fn receive() -> Result<()> {
    let sock = create_recv_sock()?;
    let mut buf = [0u8; 576];

    loop {
        let (_bytes_recvd, _ip_addr) =
            match timeout(Duration::from_secs(1), sock.recv_from(&mut buf)).await {
                Ok(result) => result.unwrap(),
                Err(_) => {
                    break;
                }
            };

        let tcp_packet = match TcpPacket::new(&buf[IP_HDR_LEN as usize..]) {
            Some(packet) => packet,
            None => bail!("couldn't make tcp packet"),
        };

        let port = tcp_packet.get_source();

        match tcp_packet.get_flags() {
            18 => println!("{port}: open"),
            20 => println!("{port}: closed"),
            _ => {}
        }
    }

    Ok(())
}
