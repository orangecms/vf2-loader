use log::info;
use std::fs::File;
use xmodem::Xmodem;

fn main() {
    env_logger::init();
    info!("Sending fw.bin ...");

    let ports = serialport::available_ports().expect("No serial ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let dev = "/dev/ttyUSB0";

    let file = "./fw.bin";
    let mut file = File::open(file).unwrap();

    let mut port = serialport::new(dev, 115_200)
        .timeout(std::time::Duration::from_secs(1))
        .open()
        .expect("Failed to open port");

    let mut x = Xmodem::new();
    x.send(&mut port, &mut file).unwrap();
}
