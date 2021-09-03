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

    layout.text(&ram, &flash, true).unwrap();
}

//////////////////////////////////////////////////////////////////////////////
//
// It might be worth it to expose a public macro to create a linker script in a more idiomatic and
// readable way rather than performing just function calls. This would not exclude the current idea
// for the API, but would provide handy ergonomics to simplify the specification of the memory
// layout.
//
// #[LinkerScript]
// const LD_SCRIPT: () = {
//     enum MemoryRegions {
//         #[region(
//             address = 0x00000000,
//             size = 256.kb(),
//             access = "RX")]
//         Flash,

//         #[region(
//             address = 0x20000000,
//             size = 128.kb(),
//             access = "RWX")]
//         Ram,

//         #[region(
//             address = 0x21000000,
//             size = 16.kb(),
//             access = "RWX")]
//         CcRam,
//     }

//     enum MemoryLayout {
//         #[section(
//             region = Flash,
//             offset = 0x00,
//             size = 128.kb())]
//         VectorTable,

//         #[section(
//             region = Flash,
//             size = 128.kb())]
//         Text,

//         #[sction(
//             vma = Ram,
//             lma = Flash,
//             size = 32.kb())]
//         Ramfunc,

//         #[section(
//             vma = Ram,
//             lma = Flash,
//             size = 32.kb())]
//         Data,

//         #[section(
//             vma = Ram,
//             lma = Flash,
//             size = 32.kb())]
//         CcramData,

//         #[section(
//             region = Ram,
//             size = 32.kb())]
//         Bss,

//         #[section(
//             region = Ram,
//             size = 32.kb())]
//         CcramBss,
//     }
// };
