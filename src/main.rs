use std::fs;

use anyhow::Result;
use clap::Parser as _;
use color_print::cprintln;
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
	let mut files_modified = 0usize;
	for entry in entries {
		let path = entry.path();

		let Ok(data) = fs::read_to_string(path) else {
			continue;
		};

		if serde_json::from_str::<serde_json::Value>(&data).is_err() {
			continue;
		}

		files_open += 1;

		let mut format = match serde_json::from_str::<Format>(&data) {
			Ok(f) => f,
			Err(e) => {
				println!("failed to parse: {e}");
				errors += 1;
				continue;
			}
		};

		if modify(&mut format) {
			let file = fs::File::open(path)?;

			serde_json::to_writer(file, &format)?;

			files_modified += 1;

			cprintln!("{} - <green>modified</>", path.display());
		} else {
			cprintln!("{} - <red>not modified</>", path.display());
		}
	}

	println!("errors: {errors}");
	println!("total formats: {files_open}");
	println!("formats modified: {files_modified}");

	Ok(())
}

fn modify(_: &mut Format) -> bool {
	false
}
