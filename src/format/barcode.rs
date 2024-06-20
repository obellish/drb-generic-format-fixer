use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Position;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Barcode {
	#[serde(flatten)]
	pub position: Position,
	pub height: i32,
	#[serde(flatten)]
	extra: HashMap<String, Value>,
}
