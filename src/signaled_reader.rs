use windows::core::PCSTR;
use windows::Win32::System::Threading::{CreateEventA, EVENT_ALL_ACCESS, INFINITE, OpenEventA, WaitForSingleObject};
use windows::Win32::Foundation::{HANDLE, WAIT_OBJECT_0};
use crate::reader::Reader;

pub struct SignaledReader<T> {
    reader: Reader,
    event: HANDLE,
    callback: fn(&T),
}

impl<T> SignaledReader<T> {
    pub(crate) fn new(mmf_name: PCSTR, event_name: PCSTR, callback: fn(&T)) -> Self {
        let mut maybe_event = unsafe {
            OpenEventA(
                EVENT_ALL_ACCESS,
                false,
                event_name,
            )
        };
        
        if maybe_event.is_err() {
            maybe_event = unsafe { CreateEventA(None, false, false, event_name) };
            if maybe_event.is_err() {
                panic!("Failed to create event");
            }
        }
        
        let event = maybe_event.unwrap();

        let reader = Reader::new::<T>(mmf_name);

        SignaledReader { reader, event, callback }
    }
    
    pub(crate) fn run(&self) {
        loop {
            let wait_result = unsafe {
                WaitForSingleObject(self.event, INFINITE)
            };
            if wait_result == WAIT_OBJECT_0 {
                let data = self.reader.read::<T>();
                (self.callback)(data);
            }
        }
    }
}
