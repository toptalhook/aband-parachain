use super::*;
use codec::{Encode, Decode};
use sp_runtime::traits::{AccountIdConversion};
use sp_runtime::TypeId;

#[derive(Encode, Decode)]
pub struct ServerId<Id>(pub Id);

impl<Id> TypeId for ServerId<Id> {
	const TYPE_ID: [u8; 4] = *b"band";
}

impl<Id> From<Id> for ServerId<Id> {
	fn from(value: Id) -> Self {
		Self(value)
	}
}
