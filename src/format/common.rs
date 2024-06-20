use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Position {
	#[serde(rename = "xPos")]
	pub x: i32,
	#[serde(rename = "yPos")]
	pub y: i32,
}
