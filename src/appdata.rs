use crate::constants::CHROMA_STRING_SIZE;

#[repr(C, packed)]
pub struct ChromaAppData {
    app_count: u32,
    unused: [u16; CHROMA_STRING_SIZE],
    current_app_id: u32,
    padding: u32,
    app_info: [ChromaAppInfo; 50],
}

impl ChromaAppData {
    pub fn get_current_app_name(&self) -> String {
        for app in self.app_info.iter() {
            if app.app_id == self.current_app_id {
                let name = app.app_name;
                return String::from_utf16_lossy(&name);
            }
        }
        String::new()
    }
}

#[repr(C, packed)]
pub struct ChromaAppInfo {
    app_name: [u16; CHROMA_STRING_SIZE],
    app_id: u32,
    padding: u32,
}