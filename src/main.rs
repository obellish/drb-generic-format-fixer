use std::{fs::OpenOptions, io::Seek, thread, time::Duration};

use anyhow::Result;
use clap::Parser as _;
use color_print::cprintln;
use drb_generic_format_fixer::{
	format::{CharSize, DataSource, Format, Position},
	Args,
};
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

	println!("Size of format: {} bytes", size_of::<Format>());

	thread::sleep(Duration::from_secs(2));

	let entries = WalkDir::new(args.input_folder)
		.max_depth(2)
		.into_iter()
		.filter_map(std::result::Result::ok);

	let mut errors = 0usize;
	let mut files_open = 0usize;
	let mut files_modified = 0usize;
	for entry in entries {
		let path = entry.path();

		let Ok(mut file) = OpenOptions::new().read(true).write(true).open(path) else {
			continue;
		};

		if serde_json::from_reader::<_, serde_json::Value>(&file).is_err() {
			continue;
		}

		files_open += 1;
		file.rewind()?;

		let mut format = match serde_json::from_reader::<_, Format>(&file) {
			Ok(f) => f,
			Err(e) => {
				cprintln!("{} - <r!>{e}", path.display());
				errors += 1;
				continue;
			}
		};

		file.rewind()?;

		if modify(&mut format) {
			file.set_len(0)?;

			serde_json::to_writer(file, &format)?;

			files_modified += 1;

			cprintln!("{} - <b!>modified", path.display());
		} else {
			cprintln!("{} - <g!>not modified", path.display());
		}
	}

	println!("errors: {errors}");
	println!("total formats: {files_open}");
	println!("formats modified: {files_modified}");

	Ok(())
}

const DEFAULT_CHAR_SIZE: CharSize = CharSize {
	width: 55,
	height: 35,
};

fn modify(format: &mut Format) -> bool {
	let original = format.clone();

	// Barcodes
	format.barcodes[0].position = Position { x: 140, y: 515 };
	format.barcodes[0].height = 113;

	format.barcodes[1].position = Position { x: 140, y: 735 };
	format.barcodes[1].height = 113;

	// Text
	let mut first_text = true;

	for text in format
		.texts
		.iter_mut()
		.filter(|t| matches!(t.data_source, DataSource::Fixed) && t.data == "FastPass")
	{
		if first_text {
			text.position = Position { x: 145, y: 640 };
			first_text = false;
		} else {
			text.position = Position { x: 145, y: 860 };
		}
		text.size = DEFAULT_CHAR_SIZE;
	}

	*format != original
}
