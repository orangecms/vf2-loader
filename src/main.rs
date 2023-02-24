use log::info;
use std::fs::{metadata, File};
use std::io::{stdout, Write};
use xmodem::{Xmodem, XmodemCfg};

fn main() {
    env_logger::init();
    info!("Sending fw.bin ...");

    let ports = serialport::available_ports().expect("No serial ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
    // TODO: Do not hardcode this
    let dev = "/dev/ttyUSB0";
    // TODO: Do not hardcode this
    let file = "./fw.bin";
    let size = metadata(file).unwrap().len();
    let mut file = File::open(file).unwrap();

    let mut port = serialport::new(dev, 115_200)
        .timeout(std::time::Duration::from_secs(5))
        .open()
        .expect("Failed to open port");

    const PACKET_SIZE: u32 = 128;
    let progress = |p: u32| {
        let percent = (p * PACKET_SIZE * 100) as f64 / size as f64;
        if p % 4 == 0 {
            print!("➡️ ");
        }
        if p % 80 == 0 {
            println!("{p:06} ({percent:04.1}%)");
        }
        stdout().flush().ok();
    };
    let buf: &mut [u8; 1] = &mut [0 as u8];
    while buf[0] != b'C' {
        port.read_exact(buf).ok();
    }
    let mut x = Xmodem::new(XmodemCfg {
        max_errors: 30,
        pad_byte: 0x1a,
        block_length: xmodem::BlockLength::Standard,
        checksum_mode: xmodem::Checksum::CRC16,
        packet_callback: Some(&progress),
    });
    x.send(&mut port, &mut file).unwrap();
    println!("Done.");
}
