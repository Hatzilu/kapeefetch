use std::env;
use std::io;

extern crate sys_info;
use sys_info::LinuxOSReleaseInfo;

fn main() {
	let os_info = sys_info::os_release().expect("Failed to get os info");
	println!("Name: {}", os_info);
	println!("Host: {}", sys_info::hostname().expect("Unknown hostname"));
	let os_release_info = sys_info::linux_os_release().expect("Failed 2");

	match os_release_info.name {
		Some(name) => println!("OS name: {}",name),
		None => println!("OOPSIE!@!!@"),
}
//	let os_name = sys_info::LinuxOSReleaseInfo::name();
//	println!("osname is {os_name:?}");
}
