use clap::Parser;
use log::info;
use std::fs::{metadata, File};
use std::io::{stdout, Write};
use xmodem::{Xmodem, XmodemCfg};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Serial port
    #[arg(short, long, default_value = "/dev/ttyUSB0")]
    dev: String,

    /// Firmware file
    #[arg(index = 1)]
    file: String,
}
fn main() {
    let Args { dev, file } = Args::parse();

    env_logger::init();
    info!("Sending {file}...");

    let size = metadata(file.clone()).unwrap().len();
    let mut file = File::open(file).unwrap();

    let mut port = match serialport::new(dev.clone(), 115_200)
        .timeout(std::time::Duration::from_secs(1))
        .open()
    {
        Ok(d) => d,
        Err(_) => {
            println!("Failed to open serial port {dev}; available ports:");
            let ports = serialport::available_ports().expect("No serial ports found!");

            for p in ports {
                println!("{}", p.port_name);
            }
            std::process::exit(1);
        }
    };

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
    let buf: &mut [u8; 1] = &mut [0];
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
