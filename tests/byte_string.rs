use rlua::Lua;
use rlua::ByteString;

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
        let isi = globals
            .get::<_, ByteString>("invalid_sequence_identifier")
            .unwrap();
        assert_eq!(isi, [0xa0, 0xa1]);
        let i2os2 = globals
            .get::<_, ByteString>("invalid_2_octet_sequence_2nd")
            .unwrap();
        assert_eq!(i2os2, [0xc3, 0x28]);
        let i3os2 = globals
            .get::<_, ByteString>("invalid_3_octet_sequence_2nd")
            .unwrap();
        assert_eq!(i3os2, [0xe2, 0x28, 0xa1]);
        let i3os3 = globals
            .get::<_, ByteString>("invalid_3_octet_sequence_3rd")
            .unwrap();
        assert_eq!(i3os3, [0xe2, 0x82, 0x28]);
        let i4os2 = globals
            .get::<_, ByteString>("invalid_4_octet_sequence_2nd")
            .unwrap();
        assert_eq!(i4os2, [0xf0, 0x28, 0x8c, 0xbc]);
        let i4os3 = globals
            .get::<_, ByteString>("invalid_4_octet_sequence_3rd")
            .unwrap();
        assert_eq!(i4os3, [0xf0, 0x90, 0x28, 0xbc]);
        let i4os4 = globals
            .get::<_, ByteString>("invalid_4_octet_sequence_4th")
            .unwrap();
        assert_eq!(i4os4, [0xf0, 0x28, 0x8c, 0x28]);
        let aas = globals.get::<_, ByteString>("an_actual_string").unwrap();
        assert_eq!(aas, b"Hello, world!");
    });
}
