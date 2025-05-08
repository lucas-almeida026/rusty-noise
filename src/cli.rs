use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "A simple noise generator written in Rust.", long_about = None)]
pub struct Args {
	/// White noise volume (0-100)
	#[arg(short = 'w', num_args(0..=1), long, default_missing_value = "0", default_value_t = 0.0)]
	pub white: f32,

	/// Brown noise volume (0-100)
	#[arg(short = 'r', num_args(0..=1), long, default_missing_value = "0", default_value_t = 0.0)]
	pub brown: f32,

	/// Pink noise volume (0-100)
	#[arg(short = 'p', num_args(0..=1), long, default_missing_value = "0", default_value_t = 0.0)]
	pub pink: f32,

	/// Blue noise volume (0-100)
	#[arg(short = 'b', num_args(0..=1), long, default_missing_value = "0", default_value_t = 0.0)]
	pub blue: f32,

	/// Master volume (0-100)
	#[arg(short = 'm', long = "master-volume", default_missing_value = "0", default_value_t = 0.0)]
	pub master_volume: f32
}