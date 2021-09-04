use ld_script_macros::linker_script;

linker_script! {}

#[test]
fn test() {
    assert_eq!(VAR, 123);
}
