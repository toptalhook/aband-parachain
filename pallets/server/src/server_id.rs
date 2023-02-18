use super::*;
use codec::{Decode, Encode};
use sp_runtime::{traits::AccountIdConversion, TypeId};

#[derive(Encode, Decode)]
pub struct ServerId<Id>(pub Id);

impl<Id> TypeId for ServerId<Id> {
	const TYPE_ID: [u8; 4] = *b"serv";
}

impl<Id> From<Id> for ServerId<Id> {
	fn from(value: Id) -> Self {
		Self(value)
	}
}
