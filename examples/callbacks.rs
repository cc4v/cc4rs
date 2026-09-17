use cc4rs::*;
use std::sync::atomic::{AtomicBool, Ordering};

static MOUSE_DOWN: AtomicBool = AtomicBool::new(false);

fn main() {
    on_init!(setup);
    on_exit!(on_exit);
    on_key_pressed!(on_key_pressed);
    on_key_released!(on_key_released);
    on_mouse_pressed!(on_mouse_pressed);
    on_mouse_released!(on_mouse_released);
    on_mouse_moved!(on_mouse_moved);
    run(draw);
}

fn setup() {
    println!("setup");
}

fn draw() {
    fill();
    let color = if MOUSE_DOWN.load(Ordering::Relaxed) {
        colors::red()
    } else {
        colors::gray()
    };
    set_color(color);
    rect(20.0, 20.0, 100.0, 100.0);
}

fn on_exit() {
    println!("exit");
}

fn on_key_pressed(keycode: Keycode, modifiers: Modifiers) {
    println!("key pressed: {keycode:?}, modifiers: {modifiers}");
}

fn on_key_released(keycode: Keycode, modifiers: Modifiers) {
    println!("key released: {keycode:?}, modifiers: {modifiers}");
}

fn on_mouse_pressed(x: f32, y: f32, button: Mousebutton) {
    MOUSE_DOWN.store(true, Ordering::Relaxed);
    println!("mouse pressed: x={x}, y={y}, button={button:?}");
}

fn on_mouse_released(x: f32, y: f32, button: Mousebutton) {
    MOUSE_DOWN.store(false, Ordering::Relaxed);
    println!("mouse released: x={x}, y={y}, button={button:?}");
}

fn on_mouse_moved(x: f32, y: f32) {
    let _ = (x, y);
}
