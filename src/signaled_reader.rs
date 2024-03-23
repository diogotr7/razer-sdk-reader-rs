use windows::core::PCSTR;
use windows::Win32::Foundation::{HANDLE, WAIT_OBJECT_0};
use windows::Win32::System::Threading::{
    CreateEventA, OpenEventA, WaitForSingleObject, EVENT_ALL_ACCESS, INFINITE,
};

use crate::reader::Reader;

pub struct SignaledReader<T> {
    reader: Reader,
    event: HANDLE,
    callback: Box<dyn Fn(&T) + Send>,
}

impl<T> SignaledReader<T> {
    pub(crate) fn new(
        mmf_name: PCSTR,
        event_name: PCSTR,
        callback: Box<dyn Fn(&T) + Send>,
    ) -> Self {
        let event = unsafe {
            OpenEventA(EVENT_ALL_ACCESS, false, event_name)
                .unwrap_or_else(|_| CreateEventA(None, false, false, event_name).unwrap())
        };

        let reader = Reader::new::<T>(mmf_name);

        SignaledReader {
            reader,
            event,
            callback,
        }
    }

    pub(crate) fn run(&self) {
        loop {
            let wait_result = unsafe { WaitForSingleObject(self.event, INFINITE) };
            if wait_result == WAIT_OBJECT_0 {
                let data = self.reader.read::<T>();
                (self.callback)(data);
            }
        }
    }
}
