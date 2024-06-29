use std::io::stdin;
use std::time::Duration;
use std::path::PathBuf;
use clap::Parser;
use dbus::blocking::Connection;

mod dbusgen;
use dbusgen::login1::OrgFreedesktopLogin1Manager;

/// Bad Apple!! but with nested Wayland compositors
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
	/// Path to directory containing the image sequence
	images: PathBuf,
	/// Path to the audio file
	audio: PathBuf,
	
	/// FPS as divisor of 29.97
	#[arg(long, default_value_t = 2)]
	fps: u8,
	/// Parent compositor width
	#[arg(long, default_value_t = 640)]
	parent_width: u16,
	/// Parent compositor height
	#[arg(long, default_value_t = 480)]
	parent_height: u16,
	/// Individual compositor size
	#[arg(long, default_value_t = 32)]
	pixel_size: u16,
}

// fn inhibit() {}

fn main() {
    let _ = Args::parse();

	let system_bus = Connection::new_system().unwrap();

	let login1 = system_bus.with_proxy(
		"org.freedesktop.login1",
		"/org/freedesktop/login1",
		Duration::new(5, 0)
	);
	
	let inhibitor = login1.inhibit(
		"sleep",
		"Bad Apple!!",
		"Playing Bad Apple!!",
		"block"
	).unwrap();

	println!("press enter to stop inhibiting");
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	drop(inhibitor);
}
