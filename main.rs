use std::process::Command;
use sysinfo::{System};
use colored::*;

fn main() {
    let mut sys = System::new_all();
    println!("{}", "L       III  N   N  U   U  X   X".yellow());
    println!("{}", "L        I   NN  N  U   U   X X".yellow());
    println!("{}", "L        I   N N N  U   U    X".yellow());
    println!("{}", "L        I   N  NN  U   U   X X".yellow());
    println!("{}", "LLLLL   III  N   N  UUUUU  X   X".yellow());
    println!("-------------------------");
    Command::new("whoami")
        .spawn();
    sys.refresh_all();
    println!("------------------");
    println!("{}{:?}", "OS:             ".yellow(), System::name());
    println!("{}{:?}", "Kernel:         ".yellow(), System::kernel_version());
    println!("{}{:?} (Seconds)", "Uptime:         ".yellow(), System::uptime());
}
