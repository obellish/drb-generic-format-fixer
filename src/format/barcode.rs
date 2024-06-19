use serde::{Deserialize, Serialize};

use super::{Orientation, Position};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Barcode {
	#[serde(rename = "barcodeFont")]
	pub font: String,
	pub orientation: Orientation,
	#[serde(flatten)]
	pub position: Position,
	pub mod43: bool,
	pub height: i32,
	pub interpretation: bool,
	pub interp_above: bool,
	pub data: String,
	pub data_source: String,
	pub module_width: i32,
	pub wide_narrow_ratio: String,
}
