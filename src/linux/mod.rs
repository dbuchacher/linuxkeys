use crate::VD;
pub use std::{process, error::Error, thread};
use evdev::{Device, InputEvent, EventType};
pub use evdev::Key; 

pub mod keys;
pub mod hook;
pub mod hotkey;
pub mod virtual_keyboard;

// find hardware devices connected to pc
pub fn find_devices() -> Vec<Device> {
    // this will be returned after finding all useable devices
    let mut useable: Vec<Device> = Vec::new();

    // get all devices on system
    let devices = evdev::enumerate().map(|t| t).collect::<Vec<_>>(); 

    // devices that have an `enter key` or a `left click` will be added to return vector
    for device in devices {
        if device.supported_keys().map_or(false, |keys| { 
            keys.contains(Key::KEY_ENTER) ||
            keys.contains(Key::BTN_LEFT)
        }) {
            useable.push(device);
        } 
    }

    useable
}

pub trait Actions {
    fn state(&self) -> bool;
    fn send(&self, state: u8);
}

impl Actions for Key {
    // find the current postion of a key
    fn state(&self) -> bool {
    // find devices attached to pc
        let devices = find_devices();

    // return true when key is down
        for device in devices {
            if device.get_key_state().unwrap().contains(*self) {
                return true
            }
        }
    // else return false when key is up
        false
    }

    // sends a key: 0 is up, 1 is down, 2 in down then up
    fn send(&self, state: u8) {
        match state {
            0 => VD.lock().unwrap().emit(&[InputEvent::new(EventType::KEY, self.0, 0)]).unwrap(),
            1 => VD.lock().unwrap().emit(&[InputEvent::new(EventType::KEY, self.0, 1)]).unwrap(),
            2 => VD.lock().unwrap().emit(&[
                InputEvent::new(EventType::KEY, self.0, 1),
                InputEvent::new(EventType::KEY, self.0, 0)
            ]).unwrap(),
            _ => eprintln!("Failed Sending Keystroke"),
        }
    }
}