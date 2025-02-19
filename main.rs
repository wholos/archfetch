use std::process::Command;
use sysinfo::{System};

fn main() {
	println!(" ##  RCH");
	println!("#  # rch");
	println!("#### RCH");
	println!("#  # rch");
	println!("#  # RCH");
    Command::new("whoami")
        .spawn()
        .expect("Error!");

	let mut sys = System::new_all();

	sys.refresh_all();

	println!("=> system:");
	println!("System name:             {:?}", System::name());
	println!("System kernel version:   {:?}", System::kernel_version());
	println!("System OS version:       {:?}", System::os_version());
	println!("System host name:        {:?}", System::host_name());
}
