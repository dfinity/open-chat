use ic_cdk::init;
use types::{UserId, UserType};

use crate::{canister_id_from_u64, memory::get_stable_memory_map_memory, model::direct_chat::DirectChat, TEST_STATE};

#[init]
fn init() {
    stable_memory_map::init(get_stable_memory_map_memory());
    TEST_STATE.with_borrow_mut(|chat| {
        *chat = Some(DirectChat::new(
            UserId::new(canister_id_from_u64(1)),
            UserType::User,
            None,
            1,
            0,
        ))
    })
}
