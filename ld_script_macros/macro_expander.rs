use ld_script_macros::linker_script;

linker_script! {
    MemoryRegions => {
        Flash => {
             address = 0x00000000,
             size = 256.kb(),
             access = "RX",
        },
        Ram => {
             address = 0x20000000,
             size = 128.kb(),
             access = "RWX",

        },
        CcRam => {
             address = 0x21000000,
             size = 16.kb(),
             access = "RWX",
        },
    },

    Sections => {
        VectorTable => {
            region = Flash,
            offset = 0x00,
            size = 128.kb(),
        },

        Text => {
            region = Flash,
            size = 128.kb(),
        },

        Ramfunc => {
            vma = Ram,
            lma = Flash,
            size = 32.kb(),
        },

        Data => {
            vma = Ram,
            lma = Flash,
            size = 32.kb(),
        },

        CcramData => {
            vma = Ram,
            lma = Flash,
            size = 32.kb(),
        },

        Bss => {
            region = Ram,
            size = 32.kb(),
        },

        CcramBss => {
            region = Ram,
            size = 32.kb(),
        },
    },
}

fn main() {}
