use sysinfo::{System};
use colored::*;

fn main() {
	println!("{}", " ##  ####  ### #  #".blue());
	println!("{}", "#  # #  # #    #  #".blue());
	println!("{}", "#### #### #    ####".blue());
	println!("{}", "#  # ###  #    #  #".blue());
	println!("{}", "#  # # ##  ### #  #".blue());

	let mut sys = System::new_all();

	sys.refresh_all();

	println!("=> system:");
	println!("󰇄 System name:             {:?}", System::name());
	println!(" System kernel version:   {:?}", System::kernel_version());
	println!("󰇅 System OS version:       {:?}", System::os_version());
	println!(" System host name:        {:?}", System::host_name());
}
