use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rfid {
	#[serde(rename = "encodeEPC")]
	pub encode_epc: bool,
	pub epc_source: String,
	#[serde(rename = "epcVal")]
	pub epc_value: String,
	pub epc_lock: bool,
	#[serde(default)]
	pub pc_word: String,
	#[serde(default)]
	pub epc_format: EncodeFormat,
	pub encode_access: bool,
	#[serde(rename = "accessVal")]
	pub access_password: String,
	#[serde(rename = "killVal", default)]
	pub kill_password: String,
	pub access_lock: bool,
	#[serde(default)]
	pub kill_lock: bool,
	pub encode_user: bool,
	pub user_source: String,
	#[serde(rename = "userVal")]
	pub user_value: String,
	#[serde(default)]
	pub user_format: EncodeFormat,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EncodeFormat {
	#[default]
	Hex,
	#[serde(other)]
	Other,
}
