use std::{fs::OpenOptions, io::Seek, thread, time::Duration};

use anyhow::Result;
use clap::Parser as _;
use color_print::cprintln;
use drb_generic_format_fixer::{
	format::{Barcode, CharSize, DataSource, Format, Position, Text},
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
	let mut skipped = 0usize;
	let mut files_open = 0usize;
	let mut files_modified = 0usize;
	for entry in entries {
		let path = entry.path();
		let displayable = path.display();

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
				cprintln!("{displayable} - <r!>{e}");
				errors += 1;
				continue;
			}
		};

		if displayable.to_string().to_lowercase().contains("custom") {
			cprintln!("{displayable} - <r!>found custom format");
			skipped += 1;
			continue;
		}

		let barcode_count = format.barcodes.len();

		if barcode_count != 2 {
			cprintln!("{displayable} - <r!>expected 2 barcodes, got {barcode_count}");
			skipped += 1;
			continue;
		}

		if format
			.barcodes
			.iter()
			.any(|b| b.position.x >= 1300 || b.position.y >= 1300)
		{
			cprintln!("{displayable} - <r!>barcode outside visible range");
			skipped += 1;
			continue;
		}

		file.rewind()?;

		if modify(&mut format) {
			file.set_len(0)?;

			serde_json::to_writer(file, &format)?;

			files_modified += 1;

			cprintln!("{displayable} - <b!>modified");
		} else {
			cprintln!("{displayable} - <g!>not modified");
		}
	}

	println!("errors: {errors}");
	println!("skipped: {skipped}");
	println!("total formats: {files_open}");
	println!("formats modified: {files_modified}");

	Ok(())
}

fn modify(format: &mut Format) -> bool {
	let original = format.clone();

	// Barcodes
	modify_barcodes(&mut format.barcodes);

	// Text
	modify_fastpass_text(&mut format.texts);
	modify_index_text(&mut format.texts);
	modify_part_number_text(&mut format.texts);
	modify_human_readable_text(&mut format.texts);

	*format != original
}

fn modify_barcodes(barcodes: &mut [Barcode]) {
	barcodes[0].position = Position { x: 140, y: 515 };
	barcodes[0].height = 113;

	barcodes[1].position = Position { x: 140, y: 735 };
	barcodes[1].height = 113;
}

const DEFAULT_CHAR_SIZE: CharSize = CharSize {
	width: 55,
	height: 35,
};

fn modify_fastpass_text(texts: &mut [Text]) {
	let mut first_text = true;

	for text in texts
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
}

fn modify_human_readable_text(texts: &mut [Text]) {
	let mut first_text = true;

	for text in texts
		.iter_mut()
		.filter(|t| matches!(t.data_source, DataSource::HumanReadable))
	{
		if first_text {
			text.position = Position { x: 380, y: 640 };
			first_text = false;
		} else {
			text.position = Position { x: 380, y: 860 };
		}
		text.size = DEFAULT_CHAR_SIZE;
	}
}

fn modify_index_text(texts: &mut [Text]) {
	let index = texts
		.iter_mut()
		.find(|t| matches!(t.data_source, DataSource::Index));

	if let Some(index) = index {
		index.position = Position { x: 85, y: 85 };
		index.size = CharSize {
			width: 30,
			height: 30,
		};
	}
}

fn modify_part_number_text(texts: &mut [Text]) {
	let part_num = texts.iter_mut().find(|t| {
		matches!(
			t.data_source,
			DataSource::Partnum | DataSource::PartnumShorthand
		)
	});

	if let Some(part_num) = part_num {
		part_num.position = Position { x: 300, y: 360 };
		part_num.size = CharSize {
			width: 35,
			height: 35,
		};
	}
}
