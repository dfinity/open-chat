use canister_api_macros::query;
use chat_events::{MessageContentInternal, PushMessageArgs, TextContentInternal};
use event_store_producer::NullRuntime;
use types::{MessageId, UserId};

use crate::{canister_id_from_u64, TEST_STATE};

#[query(candid = true)]
fn go() {
    TEST_STATE.with_borrow_mut(|chat| {
        let chat = chat.as_mut().unwrap();
        for i in 0..1000 {
            let _wrapper = chat.push_message::<NullRuntime>(
                i % 2 == 1,
                PushMessageArgs {
                    sender: UserId::new(canister_id_from_u64(i % 2)),
                    thread_root_message_index: None,
                    message_id: MessageId::from(i),
                    content: MessageContentInternal::Text(TextContentInternal {
                        text: std::iter::repeat_n(format!("{i}"), 200).collect(),
                    }),
                    bot_context: None,
                    mentioned: vec![UserId::new(canister_id_from_u64(1))],
                    replies_to: None,
                    forwarded: false,
                    sender_is_bot: false,
                    block_level_markdown: false,
                    correlation_id: 0,
                    now: i,
                },
                None,
                None,
            );
        }
    });
}
