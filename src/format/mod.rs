mod barcode;
mod common;
mod text;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub use self::{barcode::Barcode, common::*, text::Text};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Format {
	#[serde(rename = "labelBarcodes")]
	pub barcodes: Vec<Barcode>,
	#[serde(rename = "labelTexts")]
	pub texts: Vec<Text>,
	#[serde(flatten)]
	extra: HashMap<String, Value>,
}
