use rlua::Lua;

use bytes::{Bytes, BytesMut};

#[test]
fn byte_string_views() {
    Lua::new().context(|lua| {
        lua.load(
            r#"
                invalid_sequence_identifier = "\xa0\xa1"
                invalid_2_octet_sequence_2nd = "\xc3\x28"
                invalid_3_octet_sequence_2nd ="\xe2\x28\xa1"
                invalid_3_octet_sequence_3rd = "\xe2\x82\x28"
                invalid_4_octet_sequence_2nd = "\xf0\x28\x8c\xbc"
                invalid_4_octet_sequence_3rd = "\xf0\x90\x28\xbc"
                invalid_4_octet_sequence_4th = "\xf0\x28\x8c\x28"

                an_actual_string = "Hello, world!"
            "#,
        )
        .exec()
        .unwrap();

        let globals = lua.globals();
        globals
            .get::<_, Bytes>("invalid_sequence_identifier")
            .unwrap();
        globals
            .get::<_, Bytes>("invalid_2_octet_sequence_2nd")
            .unwrap();
        globals
            .get::<_, Bytes>("invalid_3_octet_sequence_2nd")
            .unwrap();
        globals
            .get::<_, Bytes>("invalid_3_octet_sequence_3rd")
            .unwrap();
        globals
            .get::<_, Bytes>("invalid_4_octet_sequence_2nd")
            .unwrap();
        globals
            .get::<_, Bytes>("invalid_4_octet_sequence_3rd")
            .unwrap();
        globals
            .get::<_, Bytes>("invalid_4_octet_sequence_4th")
            .unwrap();
        globals.get::<_, Bytes>("an_actual_string").unwrap();

        globals
            .get::<_, BytesMut>("invalid_sequence_identifier")
            .unwrap();
        globals
            .get::<_, BytesMut>("invalid_2_octet_sequence_2nd")
            .unwrap();
        globals
            .get::<_, BytesMut>("invalid_3_octet_sequence_2nd")
            .unwrap();
        globals
            .get::<_, BytesMut>("invalid_3_octet_sequence_3rd")
            .unwrap();
        globals
            .get::<_, BytesMut>("invalid_4_octet_sequence_2nd")
            .unwrap();
        globals
            .get::<_, BytesMut>("invalid_4_octet_sequence_3rd")
            .unwrap();
        globals
            .get::<_, BytesMut>("invalid_4_octet_sequence_4th")
            .unwrap();
        globals.get::<_, BytesMut>("an_actual_string").unwrap();
    });
}
