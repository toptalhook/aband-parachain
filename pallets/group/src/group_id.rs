use super::*;
use codec::{Decode, Encode};
use sp_runtime::{traits::AccountIdConversion, TypeId};

#[derive(Encode, Decode)]
pub struct GroupId<Id>(pub Id);

impl<Id> TypeId for GroupId<Id> {
	const TYPE_ID: [u8; 4] = *b"grou";
}

impl<Id> From<Id> for GroupId<Id> {
	fn from(value: Id) -> Self {
		Self(value)
	}
}
