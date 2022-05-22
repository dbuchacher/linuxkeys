use linuxkeys::*;

fn main() {
    thread::sleep(std::time::Duration::from_millis(250));

    normal();

    if let Err(e) = set_hook() {
        eprintln!("Error setting hook: {}", e);
        process::exit(1);
    }
}

fn normal() {
    Hotkey::new(Q, 0).mods(vec![CAPS]).send(F11,   0).spawn();
    Hotkey::new(Q, 1).mods(vec![CAPS]).send(F11,   1).spawn();
    Hotkey::new(W, 0).mods(vec![CAPS]).send(F12,   0).spawn();
    Hotkey::new(W, 1).mods(vec![CAPS]).send(F12,   1).spawn();
    Hotkey::new(E, 1).mods(vec![CAPS]).send(F13,   1).spawn();
    Hotkey::new(E, 0).mods(vec![CAPS]).send(F13,   0).spawn();

    Hotkey::new(C, 1).mods(vec![CAPS]).send(M,   1).spawn();
    Hotkey::new(C, 0).mods(vec![CAPS]).send(M,   0).spawn();

    Hotkey::new(LBUTTON, 0).mods(vec![CAPS]).send(SIX,   0).spawn();
    Hotkey::new(LBUTTON, 1).mods(vec![CAPS]).send(SIX,   1).spawn();
    Hotkey::new(RBUTTON, 0).mods(vec![CAPS]).send(SEVEN, 0).spawn();
    Hotkey::new(RBUTTON, 1).mods(vec![CAPS]).send(SEVEN, 1).spawn();
    Hotkey::new(BUTTON1, 0).mods(vec![CAPS]).send(EIGHT, 0).spawn();
    Hotkey::new(BUTTON1, 1).mods(vec![CAPS]).send(EIGHT, 1).spawn();
    Hotkey::new(BUTTON2, 0).mods(vec![CAPS]).send(NINE,  0).spawn();
    Hotkey::new(BUTTON2, 1).mods(vec![CAPS]).send(NINE,  1).spawn();
    Hotkey::new(MBUTTON, 0).mods(vec![CAPS]).send(ZERO,  0).spawn();
    Hotkey::new(MBUTTON, 1).mods(vec![CAPS]).send(ZERO,  1).spawn();

    Hotkey::new(LBUTTON, 0).mods(vec![F]).send(ONE,   0).spawn();
    Hotkey::new(LBUTTON, 1).mods(vec![F]).send(ONE,   1).spawn();
    Hotkey::new(RBUTTON, 0).mods(vec![F]).send(TWO,   0).spawn();
    Hotkey::new(RBUTTON, 1).mods(vec![F]).send(TWO,   1).spawn();
    Hotkey::new(BUTTON1, 0).mods(vec![F]).send(THREE, 0).spawn();
    Hotkey::new(BUTTON1, 1).mods(vec![F]).send(THREE, 1).spawn();
    Hotkey::new(BUTTON2, 0).mods(vec![F]).send(FOUR,  0).spawn();
    Hotkey::new(BUTTON2, 1).mods(vec![F]).send(FOUR,  1).spawn();
    Hotkey::new(MBUTTON, 0).mods(vec![F]).send(FIVE,  0).spawn();
    Hotkey::new(MBUTTON, 1).mods(vec![F]).send(FIVE,  1).spawn();

    Hotkey::new(F12, 1).code(|| process::exit(1) ).spawn(); // exit
    Hotkey::new(F11, 1).code(nothing).spawn(); // exit

    remap(ONE,  ESC);    // escape
    remap(F,    E);      // action key
    remap(CAPS, F1);     // mod key
    remap(C,    R);      // reload
    // WASD
    remap(X,    S);
    remap(S,    W);
    // thumb
    remap(LALT,  SPACE);
    remap(SPACE, LALT);
    remap(LEFT,  X);
    remap(RIGHT, X);
    remap(UP,    X);
    remap(DOWN,  X);
    // 3 keys above wasdA
    remap(Q,     F1);
    remap(W,     F2);
    remap(E,     F3);
    // un-used
    remap(TWO,   F24);
    remap(THREE, F24);
    remap(FOUR,  F24);
    remap(FIVE,  F24);
    remap(R,     F24);
}

fn nothing() {
    Hotkey::remove_all();
    Hotkey::new(F11, 1).code(normal).spawn();
}

