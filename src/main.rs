use clap::Parser;
use dbus::blocking::Connection;
use std::fs::File;
use std::io::{prelude::*, stdin, stdout, BufReader, Result, Write, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::time::Duration;

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

fn inhibit(bus: Connection) -> dbus::arg::OwnedFd {
	let login1 = bus.with_proxy(
		"org.freedesktop.login1",
		"/org/freedesktop/login1",
		Duration::new(5, 0),
	);

	return login1
		.inhibit("sleep", "Bad Apple!!", "Playing Bad Apple!!", "block")
		.unwrap();
}

fn launch_dbus() -> Result<()> {
	let path = Path::new("/tmp/badapple-dbus");
	if !path.exists() {
		println!("Launching new DBus daemon");
		let file = File::create(path)?;

		let mut cmd = Command::new("dbus-launch")
			.stdout(Stdio::from(file))
			.spawn()?;

		cmd.wait()?;

		launch_dbus()?;
	} else {
		println!("DBus already running");
		let file = File::open(path)?;
		let buf = BufReader::new(file);
		let lines: Vec<String> = buf.lines()
			.map(|line| line.expect("failed to parse line"))
			.collect();

		println!("{:?}", lines);

		
	}
	
	return Err(Error::new(ErrorKind::Other, "Failed to launch DBus"));
}

fn actions() {
	println!("Actions:");
	println!("1. Create compositors and start Bad Apple!!");
	println!("2. Start Bad Apple!! with existing compositors");
	println!("3. Start compositors only");
	println!("4. Kill stray processes");

	print!("Your action? ");
	stdout().flush().unwrap();

	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();

	match input.trim() {
		"1" => {
			println!("Creating compositors and starting Bad Apple!!");
		}
		"2" => {
			println!("Starting Bad Apple!! with existing compositors");
		}
		"3" => {
			println!("Starting compositors only");
			return;
		}
		"4" => {
			println!("Killing stray processes");
			return;
		}
		_ => {
			println!("I can break rules too. Goodbye.");
			exit(1);
		}
	}

	start_play();
}

fn start_play() {
	print!("Press enter to start Bad Apple!! ");
	stdout().flush().unwrap();
	let mut _i = String::new();
	stdin().read_line(&mut _i).unwrap();
}

fn main() -> Result<()> {
	// let _ = Args::parse();
	// let inhibitor = inhibit(Connection::new_system().unwrap());

	// actions();

	// drop(inhibitor);

	launch_dbus()?;

	Ok(())
}
