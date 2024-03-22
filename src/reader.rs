use windows::core::PCSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Memory;
use windows::Win32::System::Memory::{
    FILE_MAP_ALL_ACCESS, MEMORY_MAPPED_VIEW_ADDRESS, PAGE_READWRITE,
};

//here be dragons
unsafe impl Send for SafeMemoryMappedViewAddress {}

struct SafeMemoryMappedViewAddress {
    address: MEMORY_MAPPED_VIEW_ADDRESS,
}

pub struct Reader {
    mmf: HANDLE,
    view: SafeMemoryMappedViewAddress,
}

impl Reader {
    pub(crate) fn new<T>(mmf_name: PCSTR) -> Self {
        let maybe_mmf = unsafe {
            //todo fix perms
            Memory::CreateFileMappingA(
                None,
                None,
                PAGE_READWRITE,
                0,
                std::mem::size_of::<T>() as u32,
                mmf_name,
            )
        };
        if maybe_mmf.is_err() {
            panic!("Failed to create file mapping");
        }
        let mmf = maybe_mmf.unwrap();
        println!(
            "Mapping view of file with size: {}",
            std::mem::size_of::<T>()
        );
        let view = unsafe {
            Memory::MapViewOfFile(mmf, FILE_MAP_ALL_ACCESS, 0, 0, std::mem::size_of::<T>())
        };

        let view = SafeMemoryMappedViewAddress { address: view };

        Reader { mmf, view }
    }

    pub(crate) fn read<T>(&self) -> &T {
        let vec = unsafe {
            std::slice::from_raw_parts(
                self.view.address.Value as *const u8,
                std::mem::size_of::<T>(),
            )
        };
        let r = unsafe { &*(vec.as_ptr() as *const T) };
        let _index = vec[0];
        r
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.mmf).unwrap();
        }
    }
}
