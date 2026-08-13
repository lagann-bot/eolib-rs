//! Regression test for the character-list deserialize bug.
//!
//! The generated `deserialize` for `LoginReplyServerPacket` / `CharacterReplyServerPacket`
//! character lists previously failed to consume the `0xff` chunk separator written
//! after the character count, so every `CharacterSelectionListEntry` deserialized to
//! garbage (empty name, junk id). This round-trips a single-character list and asserts
//! the fields survive.

use eolib::{
    data::{EoReader, EoSerialize, EoWriter},
    protocol::net::server::{
        CharacterReplyServerPacket, CharacterReplyServerPacketReplyCodeData,
        CharacterReplyServerPacketReplyCodeDataOk, CharacterSelectionListEntry, LoginReply,
        LoginReplyServerPacket, LoginReplyServerPacketReplyCodeData,
        LoginReplyServerPacketReplyCodeDataOk,
    },
};

fn entry(name: &str, id: i32) -> CharacterSelectionListEntry {
    CharacterSelectionListEntry {
        name: name.to_string(),
        id,
        ..Default::default()
    }
}

#[test]
fn login_reply_character_list_roundtrips() {
    let packet = LoginReplyServerPacket {
        reply_code: LoginReply::OK,
        reply_code_data: Some(LoginReplyServerPacketReplyCodeData::OK(
            LoginReplyServerPacketReplyCodeDataOk {
                characters: vec![entry("bota", 7), entry("botb", 9)],
            },
        )),
    };

    let mut writer = EoWriter::new();
    packet.serialize(&mut writer).unwrap();

    let decoded = LoginReplyServerPacket::deserialize(&EoReader::new(writer.to_byte_array()))
        .expect("deserialize should succeed");

    let Some(LoginReplyServerPacketReplyCodeData::OK(ok)) = decoded.reply_code_data else {
        panic!(
            "expected OK reply code data, got {:?}",
            decoded.reply_code_data
        );
    };

    assert_eq!(ok.characters.len(), 2);
    assert_eq!(ok.characters[0].name, "bota");
    assert_eq!(ok.characters[0].id, 7);
    assert_eq!(ok.characters[1].name, "botb");
    assert_eq!(ok.characters[1].id, 9);
}

#[test]
fn character_reply_character_list_roundtrips() {
    let packet = CharacterReplyServerPacket {
        reply_code: eolib::protocol::net::server::CharacterReply::OK,
        reply_code_data: Some(CharacterReplyServerPacketReplyCodeData::OK(
            CharacterReplyServerPacketReplyCodeDataOk {
                characters: vec![entry("bota", 7)],
            },
        )),
    };

    let mut writer = EoWriter::new();
    packet.serialize(&mut writer).unwrap();

    let decoded = CharacterReplyServerPacket::deserialize(&EoReader::new(writer.to_byte_array()))
        .expect("deserialize should succeed");

    let Some(CharacterReplyServerPacketReplyCodeData::OK(ok)) = decoded.reply_code_data else {
        panic!(
            "expected OK reply code data, got {:?}",
            decoded.reply_code_data
        );
    };

    assert_eq!(ok.characters.len(), 1);
    assert_eq!(ok.characters[0].name, "bota");
    assert_eq!(ok.characters[0].id, 7);
}
