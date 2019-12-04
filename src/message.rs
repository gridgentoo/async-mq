// SPDX-License-Identifier: GPL-2.0
#![allow(
    unused_imports,
    clippy::extra_unused_lifetimes,
    clippy::needless_lifetimes,
    clippy::redundant_closure,
    clippy::redundant_static_lifetimes
)]
include!("../schema/model_generated.rs");

pub use model::get_root_as_message;
pub use model::{Message, MessageArgs, MessageBuilder, MessageType};

#[cfg(test)]
mod tests {
    use flatbuffers::FlatBufferBuilder;
    #[test]
    fn message_create() {
        use super::get_root_as_message;
        use super::{Message, MessageArgs, MessageType};
        let msgs = ["a", "b", "c", "d"];
        for msg in &msgs {
            let mut b = FlatBufferBuilder::new();
            let bmsg = b.create_string(msg);
            let data = Message::create(
                &mut b,
                &MessageArgs {
                    msg: Some(bmsg),
                    ..Default::default()
                },
            );
            b.finish(data, None);
            let buf = b.finished_data();
            let got = get_root_as_message(buf);
            assert_eq!(msg, &got.msg().unwrap());
            assert_eq!(0, got.id());
            assert_eq!(MessageType::Hello, got.msg_type());
            println!("mesg = {:?}", got);
        }
    }
}
