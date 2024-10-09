use std::env;
use lldpd_rs::*;

fn main() {
    // Get the interface name from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <interface_name>", args[0]);
        return;
    }
    let interface_input = &args[1];
    let chassis_id = get_remote_chassis_id(interface_input);
    match chassis_id {
        Some(chassis_id) => println!("Chassis ID: {}", chassis_id),
        None => eprintln!("No chassis ID found for interface {}", interface_input),
    }

}
