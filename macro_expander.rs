use ld_script::define_linker_script;
use ld_script::U32Ext;

use std::str::FromStr;

define_linker_script! {
    CustomLinkerScript,
    MemoryRegions => {
        Flash => {
             address = 0x00000000,
             size = 256.kilobytes(),
             access = "RX",
        },
        Ram => {
             address = 0x20000000,
             size = 128.kilobytes(),
             access = "RWX",

        },
        CcRam => {
             address = 0x21000000,
             size = 16.kilobytes(),
             access = "RWX",
        },
    },

    Sections => {
        VectorTable => {
            region = Flash,
            offset = 0x00,
            size = 128.kilobytes(),
        },

        Text => {
            region = Flash,
            size = 128.kilobytes(),
        },

        Ramfunc => {
            vma = Ram,
            lma = Flash,
            size = 32.kilobytes(),
        },

        Data => {
            vma = Ram,
            lma = Flash,
            size = 32.kilobytes(),
        },

        CcramData => {
            vma = Ram,
            lma = Flash,
            size = 32.kilobytes(),
        },

        Bss => {
            region = Ram,
            size = 32.kilobytes(),
        },

        CcramBss => {
            region = Ram,
            size = 32.kilobytes(),
        },
    },
}

fn main() {
    let path = std::path::PathBuf::from_str("./").unwrap();
    let ld = CustomLinkerScript::new(&path);
    ld.generate().ok();
    ld.generate_reset().ok();
}
