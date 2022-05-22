use linuxkeys::*;
use xdotool::window::{get_window_focus, get_window_name};

// clear hotkeys and ensure no keys are left pressed down; then start new hotkey collection
fn start(func: fn()) {
    Hotkey::remove_all(); 

    // make sure these keys are in the release state
    for key in [
        ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, ZERO,
        MINUS, LBRACE, RBRACE, UP, DOWN, LEFT, RIGHT, LWIN, LCTRL, LALT,
        LSHIFT, HOME, END, PAGEUP, PAGEDOWN, INSERT, DELETE, TAB, RALT
    ] {
        key.send(0);
    }

    println!("{:?}", func);
    func();
}

// you spend most your time in this mode
fn normal() {
    Hotkey::new(CAPS,  1).code(|| start(caps_key)).spawn();
    Hotkey::new(TAB,   1).code(|| start(tab_key)).spawn();
    Hotkey::new(GRAVE, 1).send(LWIN, 1).spawn();
    Hotkey::new(GRAVE, 0).send(LWIN, 0).spawn();
    Hotkey::new(RALT,  1).code(|| start(common_actions)).spawn(); // common actions    
    Hotkey::new(F11,   1).mods(vec![LWIN, LSHIFT]).code(|| {
        Hotkey::new(ENTER2, 1).code(|| {
            thread::spawn(|| book_sleep());
        }).spawn();
    }).spawn(); // sleep timer for audio book
}

fn main() {
    thread::sleep(std::time::Duration::from_millis(250));
    
    normal();
    
    if let Err(e) = set_hook() {
        eprintln!("Error setting hook: {}", e);
        process::exit(1);
    }
}

// create a number pad
fn numbers() {
    Hotkey::new(THREE, 0).code(|| start(normal)).spawn();
    
    remap(M,     ONE);
    remap(COMMA, TWO);
    remap(DOT,   THREE);
    remap(J,     FOUR);
    remap(K,     FIVE);
    remap(L,     SIX);
    remap(U,     SEVEN);
    remap(I,     EIGHT);
    remap(O,     NINE);
    remap(SPACE, ZERO);
}

// when tab is held down; program specific 
fn tab_key() {
    Hotkey::new(TAB,  0).code(|| start(normal)).spawn();     // exit

    if focused_window_contains("Firefox") {
        start(firefox);
    }

    if focused_window_contains("Visual Studio Code") {
        start(vscode);
    }

    if focused_window_contains("Alacritty") || focused_window_contains("root@:") {
        start(alacritty);
    }
    
}

// when capslock is held down; alot of movement keys here
fn caps_key() {
    Hotkey::new(CAPS,  0).code(|| start(normal)        ).spawn(); // exit caps
    Hotkey::new(F12,   1).code(|| process::exit(1)     ).spawn(); // exit all
    Hotkey::new(THREE, 1).code(|| start(numbers)       ).spawn(); // numbers
    Hotkey::new(F11,   1).code(find_window_name             ).spawn(); // win name
   
    remap(BACKSLASH, GRAVE);
    remap(RALT,      ESC);
    remap(A,         LCTRL);
    remap(S,         LALT);
    remap(D,         LSHIFT);
    remap(I,         UP);
    remap(K,         DOWN);
    remap(J,         LEFT);
    remap(L,         RIGHT);
    remap(U,         BACKSPACE);
    remap(O,         DELETE);
    remap(H,         HOME);
    remap(SEMICOLON, END);
    remap(COMMA,     PAGEUP);
    remap(DOT,       PAGEDOWN);
    remap(SPACE,     TAB);
}

// things like undo and copy
fn common_actions() {
    Hotkey::new(RALT, 0).code(|| start(normal)).spawn();
    LCTRL.send(1);
    remap(E, Z); // undo
    remap(R, Y); // redo
    remap(D, C); // copy
    remap(F, V); // paste
}

// check which window is active
fn focused_window_contains(app: &str) -> bool {
    // xdotool getwindowfocus getwindowname
    String::from_utf8(
        get_window_name(&String::from_utf8(
            get_window_focus("").stdout
        ).expect("Error with get_window_focus")).stdout
    ).expect("Error with get_window_name").contains(app)
}

// find a window name
fn find_window_name() {
    println!("{}", { 
        String::from_utf8(
            get_window_name(&String::from_utf8(
                get_window_focus("").stdout
            ).expect("Error with get_window_focus")).stdout
        ).expect("Error with get_window_name")
    });
}

// firefox
fn firefox() {
    Hotkey::new(TAB,  0).code(|| start(normal)).spawn();     // exit

    Hotkey::new(J,   1).code(|| { LCTRL.send(1);   PAGEUP.send(2); LCTRL.send(0) }).spawn(); // foward a tab
    Hotkey::new(L,   1).code(|| { LCTRL.send(1); PAGEDOWN.send(2); LCTRL.send(0) }).spawn(); // back a tab
    Hotkey::new(Q,   1).code(|| { LCTRL.send(1);        W.send(2); LCTRL.send(0) }).spawn(); // close tab
    Hotkey::new(N,   1).code(|| { LCTRL.send(1);        T.send(2); LCTRL.send(0) }).spawn(); // new tab
    Hotkey::new(K,   1).code(|| { LCTRL.send(1);   LBRACE.send(2); LCTRL.send(0) }).spawn(); // back a page
    Hotkey::new(I,   1).code(|| { LCTRL.send(1);   RBRACE.send(2); LCTRL.send(0) }).spawn(); // forward a page
    Hotkey::new(O,   1).code(|| {  LALT.send(1);     DOWN.send(2);  LALT.send(0) }).spawn(); // forward search engine (in bar)
    Hotkey::new(U,   1).code(|| {  LALT.send(1);       UP.send(2);  LALT.send(0) }).spawn(); // back search engine (in bar)
    Hotkey::new(ONE, 1).code(|| { LCTRL.send(1);        K.send(2); LCTRL.send(0) }).spawn(); // go to search bar
}

// vscode
fn vscode() {
    Hotkey::new(TAB,    0).code(|| start(normal)).spawn(); // exit
    Hotkey::new(D,      1).code(|| { LCTRL.send(1); LSHIFT  .send(1); K     .send(2); LCTRL.send(1); LSHIFT.send(1) }).spawn(); // delete line
    Hotkey::new(LBRACE, 1).code(|| { LCTRL.send(1); LSHIFT  .send(1); LBRACE.send(2); LCTRL.send(1); LSHIFT.send(1) }).spawn(); // fold
    Hotkey::new(RBRACE, 1).code(|| { LCTRL.send(1); LSHIFT  .send(1); RBRACE.send(2); LCTRL.send(1); LSHIFT.send(1) }).spawn(); // unfold
    Hotkey::new(W,      1).code(|| { LCTRL.send(1); S       .send(2); LCTRL .send(1); })                                           .spawn(); // save
    Hotkey::new(F,      1).code(|| { LCTRL.send(1); F       .send(2); LCTRL .send(1); })                                           .spawn(); // find
    Hotkey::new(THREE,  1).code(|| { LCTRL.send(1); B       .send(2); LCTRL .send(1); })                                           .spawn(); // side bar
    Hotkey::new(L,      1).code(|| { LCTRL.send(1); PAGEDOWN.send(2); LCTRL .send(1); })                                           .spawn(); // next tab
    Hotkey::new(J,      1).code(|| { LCTRL.send(1); PAGEUP  .send(2); LCTRL .send(1); })                                           .spawn(); // prev tab
    Hotkey::new(Q,      1).code(|| { LCTRL.send(1); W       .send(2); LCTRL .send(1); })                                           .spawn(); // quit tab
    Hotkey::new(TWO,    1).code(|| { LCTRL.send(1); GRAVE   .send(2); LCTRL .send(1); })                                           .spawn(); // focus editor
    Hotkey::new(ONE,    1).code(|| { LCTRL.send(1); ONE     .send(2); LCTRL .send(1); })                                           .spawn(); // focus terminal
}

// Alacritty
fn alacritty() {
    Hotkey::new(TAB, 0).code(|| start(normal)).spawn(); // exit
    Hotkey::new(N,   1).code(|| start(nano))  .spawn(); // nano text editor
}

// nano editor
fn nano() {
    Hotkey::new(TAB, 0).code(|| start(normal)).spawn();  // exit
    
    Hotkey::new(ONE, 1).code(|| { LCTRL.send(1); SIX.send(2); LCTRL.send(0); }).spawn(); // set mark text
    Hotkey::new(K,   1).code(|| { LALT .send(1); SIX.send(2); LALT .send(0); }).spawn(); // copy
    Hotkey::new(I,   1).code(|| { LCTRL.send(1); U  .send(2); LCTRL.send(0); }).spawn(); // paste
    Hotkey::new(W,   1).code(|| { LCTRL.send(1); S  .send(2); LCTRL.send(0); }).spawn(); // save
    Hotkey::new(Q,   1).code(|| { LCTRL.send(1); X  .send(2); LCTRL.send(0); }).spawn(); // close nano
    Hotkey::new(D,   1).code(|| { LCTRL.send(1); K  .send(2); LCTRL.send(0); }).spawn(); // delete a line
    Hotkey::new(J,   1).code(|| { LALT .send(1); U  .send(2); LALT .send(0); }).spawn(); // undo
    Hotkey::new(L,   1).code(|| { LALT .send(1); E  .send(2); LALT .send(0); }).spawn(); // redo
    Hotkey::new(F,   1).code(|| { LCTRL.send(1); W  .send(2); LCTRL.send(0); }).spawn(); // find
    Hotkey::new(N,   1).code(|| { LCTRL.send(1); W  .send(2); LCTRL.send(0); }).spawn(); // find-next
    Hotkey::new(P,   1).code(|| { LCTRL.send(1); Q  .send(2); LCTRL.send(0); }).spawn(); // find-prev

}

// sleep timer for audio books
fn book_sleep() {
    SPACE.send(2);
    thread::sleep(std::time::Duration::from_secs(1*60*15));
    SPACE.send(2);
}