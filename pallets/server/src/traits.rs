use frame_support::dispatch::DispatchResult;
use sp_runtime::DispatchError;

pub trait ServerManager<ServerId, GroupId> {
	fn try_add_new_group(server_id: ServerId, group_id: GroupId) -> DispatchResult;
	fn try_remove_old_group(server_id: ServerId, group_id: GroupId) -> DispatchResult;
}

pub trait GetServerInfo<ServerId, GroupId, AccountId> {
	fn try_server_owner(server_id: ServerId) -> Result<Option<AccountId>, DispatchError>;
	fn try_get_server_creator(server_id: ServerId) -> Result<AccountId, DispatchError>;
	fn try_get_server_account_id(server_id: ServerId) -> Result<AccountId, DispatchError>;
	fn is_at_capacity(server_id: ServerId) -> bool;
}
