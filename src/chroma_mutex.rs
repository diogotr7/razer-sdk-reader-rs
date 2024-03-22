use std::collections::VecDeque;

use windows::core::PCSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::{
    CreateEventA, CreateMutexA, EVENT_ALL_ACCESS, OpenEventA, PulseEvent,
};

use crate::constants;

pub struct ChromaMutex {
    mutexes: VecDeque<HANDLE>,
}

impl ChromaMutex {
    pub fn new() -> Self {
        let mut mutexes = VecDeque::new();
        mutexes.push_back(unsafe {
            CreateMutexA(None, true, constants::SYNAPSE_ONLINE_MUTEX).unwrap()
        });
        Self::pulse_event(constants::SYNAPSE_EVENT);
        mutexes.push_back(unsafe {
            CreateMutexA(None, true, constants::OLD_SYNAPSE_ONLINE_MUTEX).unwrap()
        });
        Self::pulse_event(constants::SYNAPSE_EVENT);
        mutexes.push_back(unsafe {
            CreateMutexA(None, true, constants::OLD_SYNAPSE_VERSION_MUTEX).unwrap()
        });
        Self::pulse_event(constants::SYNAPSE_EVENT);
        mutexes.push_back(unsafe {
            CreateMutexA(None, true, constants::CHROMA_EMULATOR_MUTEX).unwrap()
        });
        println!("Mutex created");

        ChromaMutex { mutexes }
    }

    fn pulse_event(event_name: PCSTR) {
        unsafe {
            let mut handle = OpenEventA(EVENT_ALL_ACCESS, false, event_name);
            if handle.is_err() {
                handle = CreateEventA(None, false, false, event_name);
                if handle.is_err() {
                    panic!("Failed to create event");
                }
            } else {
                let actual_handle = handle.unwrap();
                _ = PulseEvent(actual_handle);
                _ = CloseHandle(actual_handle);
            }
        }
    }
}

impl Drop for ChromaMutex {
    fn drop(&mut self) {
        while let Some(mutex) = self.mutexes.pop_front() {
            unsafe {
                _ = CloseHandle(mutex);
            }
            println!("Mutex dropped");
        }
    }
}
