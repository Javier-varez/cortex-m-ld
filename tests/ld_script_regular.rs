use ld_script::{Address, MemoryLayout, U32Ext};

#[test]
fn create_regular_ld_script() {
    let mut layout = MemoryLayout::new().unwrap();

    let flash = layout
        .add_rx_region("flash", Address::new(0x00000000), 32.kilobytes())
        .unwrap();

    let ram = layout
        .add_rwx_region("ram", Address::new(0x20000000), 256.kilobytes())
        .unwrap();

    layout.text(&ram, &flash, None).unwrap();
}

//////////////////////////////////////////////////////////////////////////////
//
// It might be worth it to expose a public macro to create a linker script in a more idiomatic and
// readable way rather than performing just function calls. This would not exclude the current idea
// for the API, but would provide handy ergonomics to simplify the specification of the memory
// layout.
//
// linker_script! {
//     MemoryRegions => {
//         Flash => {
//             address = 0x00000000,
//             size = 256.kb(),
//             access = "RX",
//         },
//         Ram => {
//             address = 0x20000000,
//             size = 128.kb(),
//             access = "RWX",
//         },
//         CcRam => {
//             address = 0x21000000,
//             size = 16.kb(),
//             access = "RWX",
//         },
//     },
//
//     Sections => {
//         VectorTable {
//             region = Flash,
//             offset = 0x00,
//             size = 128.kb(),
//         },
//
//         Text {
//             region = Flash,
//             size = 128.kb(),
//         },
//
//         Ramfunc {
//             vma = Ram,
//             lma = Flash,
//             size = 32.kb(),
//         },
//
//         Data {
//             vma = Ram,
//             lma = Flash,
//             size = 32.kb(),
//         },
//
//         CcramData {
//             vma = Ram,
//             lma = Flash,
//             size = 32.kb(),
//         }
//
//         Bss {
//             region = Ram,
//             size = 32.kb(),
//         },
//
//         CcramBss {
//             region = Ram,
//             size = 32.kb(),
//         },
//     },
// }
