use std::fs;

use anyhow::Result;
use clap::Parser as _;
use drb_generic_format_fixer::{format::Format, Args};
use walkdir::WalkDir;

#[allow(warnings)]
fn main() -> Result<()> {
	let args = match Args::try_parse() {
		Ok(args) => args,
		Err(e) => {
			println!("{e}");
			return Ok(());
		}
	};

	let entries = WalkDir::new(args.input_folder)
		.max_depth(2)
		.into_iter()
		.filter_map(std::result::Result::ok);

	let mut errors = 0usize;
	let mut files_open = 0usize;
	let mut files_succeeded = 0usize;

	for entry in entries {
		let path = entry.path();

		let Ok(data) = fs::read_to_string(path) else {
			continue;
		};

		if serde_json::from_str::<serde_json::Value>(&data).is_err() {
			continue;
		}

		files_open += 1;

		println!("{}", path.display());

		if let Err(e) = serde_json::from_str::<Format>(&data) {
			println!("failed to parse: {e}");
			errors += 1;
			continue;
		}

		files_succeeded += 1;
	}

	println!("errors: {errors}");
	println!("JSON files: {files_open}");
	println!("Format files: {files_succeeded}");

	Ok(())
}
