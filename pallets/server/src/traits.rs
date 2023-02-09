
// pub trait ServerManager<ServerId, GroupId> {
// 	fn try_add_new_group(server_id: ServerId, group_id: GroupId)
// 	fn remove_old_group(server_id: ServerId, group_id: GroupId)
//
// }

pub trait GetServerInfo<ServerId, GroupId, AccountId> {
	fn get_server_id_by_group(group_id: GroupId) -> Option<ServerId>;
	fn get_server_owner(server_id: ServerId) -> Option<AccountId>;
	fn get_server_creator(server_id: ServerId) -> AccountId;
	fn get_server_account_id(server_id: ServerId) -> AccountId;
}
