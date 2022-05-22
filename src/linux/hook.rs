use evdev::Key;

use crate::{ linux::*, hotkey::* };

// invoked in the `main` function after setting hotkeys to start event monitoring loop
pub fn set_hook() -> Result<(), Box<dyn Error>> {
    // find devices attached to pc
    let devices = find_devices();

    // used in a loop later to tell threads to wait so program doesn't close right away
    let mut handles = vec![];
    
    // cycle though all devices on pc
    for mut device in devices {
        // take control of phiscial devices: this stops events from the device as well
        device.grab()?;

        // start a thread to monitor each device
        let handle = thread::spawn(|| hook(device) );
        // add each thread to `handles` vector
        handles.push(handle);
    }

    // set threads to wait so program doesn't close right away
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

// each device gets monitored through this function in speaperated threads
fn hook(mut device: evdev::Device) {
    loop {
        for event in device.fetch_events().unwrap() {
            match event.event_type().0 {
                // 1: if no hotkeys are triggered we will send the event as normal. (or if unblock is set to true)
                1 => if check_hotkeys(event) {
                    VD.lock().unwrap().emit(&[InputEvent::new(EventType::KEY, event.code(), event.value())]).unwrap();
                }
                // 2: send a relative mouse movement
                2 => VD.lock().unwrap().emit(&[InputEvent::new(EventType::RELATIVE, event.code(), event.value())]).unwrap(),
                // if not a key stroke or a mouse movement continue checking next event.
                _ => continue,
            }
        }
    }
}

fn check_hotkeys(event: InputEvent) -> bool {
    // get global hotkey variables
    let keys = HOTKEYS.lock().unwrap().clone();

    // loop through all user set hotkeys
    for key in keys {
        // if trigger key and current event don't match check next hotkey
        if key.trigger.0 != event.code() {
            continue;
        }

        // what postion is the `hotkey` suppose to be in? and what postion is the current event key in?
        let pressed: bool = (key.state.clone().unwrap() == 1) == (event.value() == 1);
        let released: bool = (key.state.clone().unwrap() == 0) == (event.value() == 0);

        // check that all modifier keys are down
        let modifiers = !key.enable_modifiers || modifiers_are_down(key.modifiers);

        // if everything is perfect trigger hotkey after
        let logic = match event.value() {
            0 => released && modifiers,
            1 => pressed && modifiers,
            _ => return false,
        };

        // if the logic is true we can apply the proper actions to the hotkey
        if logic {
            match key.action {
                HotkeyActions::None => (),
                HotkeyActions::Code => key.code.unwrap()(),
                HotkeyActions::Send => VD.lock().unwrap().emit(&[
                    InputEvent::new(EventType::KEY, key.send.0.unwrap().0, key.send.1.unwrap()),
                ]).unwrap(),
            }
            if key.unblock { 
                return true // if unblock is set to true we will send the trigger key as well 
            }
            else {
                return false // else we block the trigger key from being sent
            }
        }
    }

    return true
}

// check if the user is holding down the mod keys
fn modifiers_are_down(keys: Vec<Key>) -> bool {
    for key in keys {
        if !key.state() { return false }
    }
    true
}