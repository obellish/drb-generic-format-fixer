mod barcode;
mod common;
mod rfid;

use serde::{Deserialize, Serialize};

pub use self::{barcode::Barcode, common::*, rfid::Rfid};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Format {
	#[serde(rename = "labelName")]
	pub name: String,
	#[serde(rename = "labelWidth")]
	pub width: i32,
	#[serde(rename = "labelHeight")]
	pub height: i32,
	#[serde(rename = "hasRFID")]
	pub has_rfid: bool,
	#[serde(rename = "rfidType")]
	pub rfid_type: String,
	#[serde(rename = "labelBarcodes")]
	pub barcodes: Vec<Barcode>,
	#[serde(rename = "labelUHFRFID")]
	pub rfid: Rfid,
	#[serde(rename = "labelGraphicBox")]
	pub graphic_boxes: Vec<serde_json::Value>,
}
