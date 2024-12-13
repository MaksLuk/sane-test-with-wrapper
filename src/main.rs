extern crate sane_scan;

use sane_scan::Sane;

fn main() {
    // Initialize the SANE library
    let sane = Sane {};

    // Get the list of devices
    let devices = sane.get_devices().expect("Failed to get device list");
    dbg!(&devices);

    // Print the list of devices
    for device in devices {
        println!("Device: {}", device.name.to_str().unwrap());
    }
}

