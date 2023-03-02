use std::env;
use std::{fs::File, io::Read, process};

fn main() {
	println!("Host: {}", get_host().expect("Failed"));
	println!("CPU: {}", get_cpu_model().expect("Failed"));

}


pub fn get_host() -> Result<String, std::io::Error> {
	let mut buf = String::new();
	File::open("/sys/devices/virtual/dmi/id/product_name")?.read_to_string(&mut buf)?;
	Ok(buf.trim().to_string())
}


pub fn get_cpu_model() -> Result<String, std::io::Error> {
	let mut buffer = String::new();
	let mut final_name = String::new();
	File::open("/proc/cpuinfo")?.read_to_string(&mut buffer)?;
	for line in buffer.lines() {
		
		if line.starts_with("model name") {
			let splitted: Vec<&str> = line.split(":").collect();
			
			if let Some(name) = splitted.get(1) {
				final_name = name.to_string().trim().to_string();
				
			}
			else {
				println!("failed");
			}

		}
	}
	return Ok(final_name);
}