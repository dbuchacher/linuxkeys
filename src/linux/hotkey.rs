use evdev::Key;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::Actions;

// allows access of gobal variable 'HOTKEYS' from when we are in the event hook
pub static HOTKEYS: Lazy<Mutex<Vec<Hotkey>>> = Lazy::new(|| Mutex::new(vec![]));

// things that can happen when a hotkey is activated
#[derive(Debug, Clone)]
pub enum HotkeyActions {
    None,
    Send,
    Code,
}

// different type of options a hotkey can contain
#[derive(Debug, Clone)]
pub struct Hotkey {
    pub trigger: Key,
    pub modifiers: Vec<Key>,
    pub action: HotkeyActions,
    pub state: Option<i32>,
    pub unblock: bool,
    pub enable_modifiers: bool,
    pub code: Option<fn ()>,
    pub send: (Option<Key>, Option<i32>),
}

impl Hotkey {
    // default values for new hotkeys
    pub fn new(key: Key, state: i32) -> Hotkey {
        Hotkey {
            trigger: key,
            modifiers: Vec::new(),
            action: HotkeyActions::None,
            state: Some(state),
            unblock: false,
            enable_modifiers: false,
            code: None,
            send: (None, None),
        }
    }
    // without spawn the hotkey won't be active
    pub fn spawn(self) {
        HOTKEYS.lock().unwrap().push(self);
    }
    // delete a hotkey
    pub fn remove(key: Key) {
        HOTKEYS.lock().unwrap().retain(|x| x.trigger != key);
    }
    // delete all hotkeys
    pub fn remove_all() {
        HOTKEYS.lock().unwrap().clear();
    }
    // add modifier keys that need to be pressed before the hotkey is pressed
    pub fn mods(mut self, key: Vec<Key>) -> Self {
        self.enable_modifiers = true;
        self.modifiers = key;
        self
    }
    // unblock the hotkey key from also being sent
    pub fn unblock(mut self) -> Self {
        self.unblock = true;
        self
    }
    // run code from an external function
    pub fn code(mut self, code: fn ()) -> Self {
        self.action = HotkeyActions::Code;
        self.code = Some(code);
        self
    }
    // key event down than up
    pub fn send(mut self, key: Key, state: i32) -> Self {
        self.action = HotkeyActions::Send;
        self.send = (Some(key), Some(state));
        self
    }
}

// makes life ez
pub struct SetKeyState;

impl SetKeyState {
    pub fn up(key: Key) { if key.state() { key.send(0) } }
    pub fn down(key: Key) { if !key.state() { key.send(1) } }
    pub fn switch(key: Key) { if key.state() { key.send(0) } if !key.state() { key.send(1) } }
}

pub fn remap(input: Key, output: Key ) {
    Hotkey::new(input, 1).send(output, 1).spawn();
    Hotkey::new(input, 0).send(output, 0).spawn();
}