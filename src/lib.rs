use std::collections::hash_map::HashMap;
use std::marker::PhantomData;

#[cfg(feature = "macros")]
pub use ld_script_macros::define_linker_script;

/// Error type for the ld_script crate.
#[derive(Debug)]
pub enum Error {
    OverlapingMemoryRegion(MemoryId),
}

#[derive(Copy, Clone, Debug)]
pub struct Address(u32);

impl Address {
    pub fn new(addr: u32) -> Self {
        Self(addr)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Size(u32);

pub trait U32Ext {
    fn bytes(self) -> Size;
    fn kilobytes(self) -> Size;
    fn megabytes(self) -> Size;
}

impl U32Ext for u32 {
    fn bytes(self) -> Size {
        Size(self)
    }
    fn kilobytes(self) -> Size {
        Size(self * 1024)
    }
    fn megabytes(self) -> Size {
        Size(self * 1024 * 1024)
    }
}

#[derive(Clone, Debug)]
pub struct MemoryId(String);

#[derive(Debug)]
pub struct RW {}

#[derive(Debug)]
pub struct RX {}

#[derive(Debug)]
pub struct RWX {}

pub trait Read {}
pub trait Write {}
pub trait Execute {}

impl Read for RW {}
impl Write for RW {}

impl Read for RX {}
impl Execute for RX {}

impl Read for RWX {}
impl Write for RWX {}
impl Execute for RWX {}

pub trait MemoryRegion {
    fn get_id(&self) -> &MemoryId;
    fn get_base_addres(&self) -> Address;
    fn get_size(&self) -> Size;
}

/// A representation of a memory region with read, write and execute permissions.
#[derive(Debug)]
pub struct Memory<Type> {
    name: MemoryId,
    base_address: Address,
    size: Size,
    _type: PhantomData<Type>,
}

// Implementation for a read write memory
impl<T> MemoryRegion for Memory<T> {
    fn get_id(&self) -> &MemoryId {
        &self.name
    }

    fn get_base_addres(&self) -> Address {
        self.base_address
    }

    fn get_size(&self) -> Size {
        self.size
    }
}

struct Section {
    vma: MemoryId,
    lma: MemoryId,
    size: Option<Size>,
}

impl Section {
    fn new(vma: MemoryId, lma: MemoryId, size: Option<Size>) -> Self {
        Section { vma, lma, size }
    }
}

/// The MemoryLayout struct represents an abstraction over a GNU linker script. It can be used to
/// generate a link.x script for your embedded device, allowing customization of the placement of
/// each section and how they are laid out in memory.
pub struct MemoryLayout {
    sections: HashMap<String, Section>,
    memory_regions: Vec<Box<dyn MemoryRegion>>,
}

impl<'a> MemoryLayout {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            sections: HashMap::new(),
            memory_regions: vec![],
        })
    }

    fn check_overlap(&self, base_address: Address, size: Size) -> Result<(), Error> {
        let start_address = base_address.0;
        let end_address = base_address.0 + size.0;
        for region in &self.memory_regions {
            let region_start = region.get_base_addres().0;
            let region_end = region.get_base_addres().0 + region.get_size().0;

            if start_address > region_start && start_address < region_end {
                return Err(Error::OverlapingMemoryRegion(region.get_id().clone()));
            }
            if end_address > region_start && end_address < region_end {
                return Err(Error::OverlapingMemoryRegion(region.get_id().clone()));
            }
            if start_address < region_start && end_address > region_end {
                return Err(Error::OverlapingMemoryRegion(region.get_id().clone()));
            }
        }
        Ok(())
    }

    fn add_region<T>(
        &mut self,
        name: &str,
        base_address: Address,
        size: Size,
    ) -> Result<Memory<T>, Error> {
        self.check_overlap(base_address, size)?;

        self.memory_regions.push(Box::new(Memory::<RWX> {
            name: MemoryId(name.to_string()),
            base_address,
            size,
            _type: PhantomData,
        }));

        Ok(Memory::<T> {
            name: MemoryId(name.to_string()),
            base_address,
            size,
            _type: PhantomData,
        })
    }

    pub fn add_rwx_region(
        &mut self,
        name: &str,
        base_address: Address,
        size: Size,
    ) -> Result<Memory<RWX>, Error> {
        self.add_region(name, base_address, size)
    }

    pub fn add_rx_region(
        &mut self,
        name: &str,
        base_address: Address,
        size: Size,
    ) -> Result<Memory<RX>, Error> {
        self.add_region(name, base_address, size)
    }

    pub fn add_rw_region(
        &mut self,
        name: &str,
        base_address: Address,
        size: Size,
    ) -> Result<Memory<RW>, Error> {
        self.add_region(name, base_address, size)
    }

    pub fn vector_table<T: Read, U: Read>(
        &mut self,
        vma: &Memory<T>,
        lma: &Memory<U>,
        size: Option<Size>,
    ) -> Result<(), Error> {
        self.sections.insert(
            "vector_table".to_owned(),
            Section::new(lma.get_id().clone(), vma.get_id().clone(), size),
        );

        Ok(())
    }

    pub fn text<T: Execute, U: Read>(
        &mut self,
        vma: &Memory<T>,
        lma: &Memory<U>,
        size: Option<Size>,
    ) -> Result<(), Error> {
        self.sections.insert(
            "text".to_owned(),
            Section::new(lma.get_id().clone(), vma.get_id().clone(), size),
        );

        Ok(())
    }

    pub fn data<T: Execute, U: Read>(
        &mut self,
        vma: &Memory<T>,
        lma: &Memory<U>,
        size: Option<Size>,
    ) -> Result<(), Error> {
        self.sections.insert(
            "data".to_owned(),
            Section::new(lma.get_id().clone(), vma.get_id().clone(), size),
        );

        Ok(())
    }

    pub fn bss<T: Execute, U: Read>(
        &mut self,
        vma: &Memory<T>,
        size: Option<Size>,
    ) -> Result<(), Error> {
        self.sections.insert(
            "bss".to_owned(),
            Section::new(vma.get_id().clone(), vma.get_id().clone(), size),
        );

        Ok(())
    }

    pub fn stack<T: Execute, U: Read>(
        &mut self,
        vma: &Memory<T>,
        lma: &Memory<U>,
        size: Option<Size>,
    ) -> Result<(), Error> {
        self.sections.insert(
            "stack".to_owned(),
            Section::new(lma.get_id().clone(), vma.get_id().clone(), size),
        );

        Ok(())
    }

    pub fn uninit<T: Execute, U: Read>(
        &mut self,
        vma: &Memory<T>,
        lma: &Memory<U>,
        size: Option<Size>,
    ) -> Result<(), Error> {
        self.sections.insert(
            "uninit".to_owned(),
            Section::new(lma.get_id().clone(), vma.get_id().clone(), size),
        );

        Ok(())
    }

    pub fn ramfunc<T: Execute, U: Read>(
        &mut self,
        vma: &Memory<T>,
        lma: &Memory<U>,
        size: Option<Size>,
    ) -> Result<(), Error> {
        self.sections.insert(
            "ramfunc".to_owned(),
            Section::new(lma.get_id().clone(), vma.get_id().clone(), size),
        );

        Ok(())
    }

    pub fn custom_section<T: Execute, U: Read>(
        &mut self,
        name: &str,
        vma: &Memory<T>,
        lma: &Memory<U>,
        size: Option<Size>,
    ) -> Result<(), Error> {
        self.sections.insert(
            name.to_owned(),
            Section::new(lma.get_id().clone(), vma.get_id().clone(), size),
        );

        Ok(())
    }

    pub fn generate(self, _output_dir: &std::path::Path) -> Result<(), Error> {
        // Generate linker script in the given directory, along with the reset code.

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Address, Error, MemoryLayout, Size};

    #[test]
    fn construct_multiple_mem_regions() {
        let mut layout = MemoryLayout::new().unwrap();
        let _ = layout
            .add_rwx_region("FLASH", Address(0x00000000), Size(1024))
            .unwrap();
        let _ = layout
            .add_rwx_region("FLASH", Address(0x10000000), Size(1024))
            .unwrap();
    }

    #[test]
    fn overlapping_memory_regions() {
        let mut layout = MemoryLayout::new().unwrap();
        let _ = layout
            .add_rwx_region("FLASH", Address(0x00000000), Size(1024))
            .unwrap();
        match layout.add_rwx_region("RAM", Address(0x00000100), Size(1024)) {
            Err(Error::OverlapingMemoryRegion(id)) => {
                assert_eq!(id.0, "FLASH")
            }
            _ => {
                panic!()
            }
        };
    }

    #[test]
    fn generate_linker_script_with_missing_sections() {
        let mut layout = MemoryLayout::new().unwrap();
        let flash = layout
            .add_rw_region("FLASH", Address(0x00000000), Size(1024))
            .unwrap();
        let ram = layout
            .add_rwx_region("RAM", Address(0x00001000), Size(1024))
            .unwrap();
        layout.text(&ram, &flash, None).unwrap();
        let path: std::path::PathBuf = "".into();
        layout.generate(&path).unwrap();
    }
}
