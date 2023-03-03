use std::{fs::File, io::Read, env::var, process::Command};

fn main() {
	println!("OS: {}", get_os_name().expect("Failed to read OS name"));
	println!("Host: {}", get_host().expect("Failed to read host machine name"));
	println!("CPU: {}", get_cpu_model().expect("Failed to read CPU model"));
	println!("Shell: {}", get_shell().expect("Failed to read shell"));

}


pub fn get_host() -> Result<String, std::io::Error> {
	let mut buf = String::new();
	File::open("/sys/devices/virtual/dmi/id/product_name")?.read_to_string(&mut buf)?;
	Ok(buf.trim().to_string())
}
pub fn get_shell() -> Result<String, std::env::VarError> {
	let final_shell;
	match var("SHELL") {
		Ok(shell) => {
			if shell.starts_with("/usr/bin/") {
				final_shell = shell.replace("/usr/bin/","").to_string()
			}
			else {
				final_shell = shell
			}
		},
		Err(e) => {
			println!("Couldn't read shell env var, err: {}",e);
			final_shell = "Unknown".to_string();
		}
	}
	Ok(final_shell.to_string())
}

pub fn get_os_name() -> Result<String, std::io::Error> {
	let mut buffer = String::new();
	let mut name_without_quote_marks = String::new();
	File::open("/etc/os-release")?.read_to_string(&mut buffer)?;
	for line in buffer.lines() {
		if line.starts_with("NAME=") {
			let splitted: Vec<&str> = line.split("=").collect();
			
			if let Some(name) = splitted.get(1) {
				// remove the quote marks
				name_without_quote_marks = name.to_string().trim().to_string();
				name_without_quote_marks.remove(0);
				name_without_quote_marks.remove(name_without_quote_marks.len() - 1);

				// add a space since we'll add the architecture in the same line
				name_without_quote_marks.push(' ')
			}
		}
	}

	let architecture;
	let output;
	let mut input = Command::new("uname");
	
	input.arg("-m");
	output = input.output().expect("Failed to read architecture info").stdout;
	architecture = String::from_utf8(output).unwrap();
	name_without_quote_marks.push_str(&architecture);

	Ok(name_without_quote_marks.trim().to_string())
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