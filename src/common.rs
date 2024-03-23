use crate::constants::CHROMA_STRING_SIZE;

#[repr(C, packed)]
pub struct ChromaDevice {
    instance: [u16; CHROMA_STRING_SIZE],
    instance_name: [u16; CHROMA_STRING_SIZE],
}
