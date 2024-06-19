pub mod format;

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
	/// The input folder to search through
	#[arg(short, long, value_name = "DIRECTORY")]
	pub input_folder: PathBuf,
}
