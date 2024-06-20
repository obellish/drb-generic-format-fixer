use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Position;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Text {
	#[serde(flatten)]
	pub position: Position,
	pub data_source: DataSource,
	pub data: String,
	#[serde(flatten)]
	pub size: CharSize,
	#[serde(flatten)]
	extra: HashMap<String, Value>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataSource {
	#[default]
	Fixed,
	#[serde(rename = "HR")]
	HumanReadable,
	Index,
	Partnum,
	#[serde(rename = "PN")]
	PartnumShorthand,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CharSize {
	#[serde(rename = "charWidth")]
	pub width: i32,
	#[serde(rename = "charHeight")]
	pub height: i32,
}
