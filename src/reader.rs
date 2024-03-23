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
        let mmf = unsafe {
            //todo fix perms
            Memory::CreateFileMappingA(
                None,
                None,
                PAGE_READWRITE,
                0,
                std::mem::size_of::<T>() as u32,
                mmf_name,
            )
            .unwrap()
        };

        let view = unsafe {
            Memory::MapViewOfFile(mmf, FILE_MAP_ALL_ACCESS, 0, 0, std::mem::size_of::<T>())
        };

        Reader {
            mmf,
            view: SafeMemoryMappedViewAddress { address: view },
        }
    }

    pub(crate) fn read<T>(&self) -> &T {
        unsafe { &*(self.view.address.Value as *const T) }
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.mmf).unwrap();
        }
    }
}
