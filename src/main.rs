use std::path::PathBuf;
use clap::Parser;

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

fn main() {
    let _ = Args::parse();
}
