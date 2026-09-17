use cc4rs as cc;
use std::sync::atomic::{AtomicBool, Ordering};

static MOUSE_DOWN: AtomicBool = AtomicBool::new(false);

fn main() {
    cc::on_init!(setup);
    cc::on_exit!(on_exit);
    cc::on_key_pressed!(on_key_pressed);
    cc::on_key_released!(on_key_released);
    cc::on_mouse_pressed!(on_mouse_pressed);
    cc::on_mouse_released!(on_mouse_released);
    cc::on_mouse_moved!(on_mouse_moved);
    cc::run(draw);
}

fn setup() {
    println!("setup");
}

fn draw() {
    cc::fill();
    let color = if MOUSE_DOWN.load(Ordering::Relaxed) {
        cc::colors::red()
    } else {
        cc::colors::gray()
    };
    cc::set_color(color);
    cc::rect(20.0, 20.0, 100.0, 100.0);
}

fn on_exit() {
    println!("exit");
}

fn on_key_pressed(keycode: cc::Keycode, modifiers: cc::Modifiers) {
    println!("key pressed: {keycode:?}, modifiers: {modifiers}");
}

fn on_key_released(keycode: cc::Keycode, modifiers: cc::Modifiers) {
    println!("key released: {keycode:?}, modifiers: {modifiers}");
}

fn on_mouse_pressed(x: f32, y: f32, button: cc::Mousebutton) {
    MOUSE_DOWN.store(true, Ordering::Relaxed);
    println!("mouse pressed: x={x}, y={y}, button={button:?}");
}

fn on_mouse_released(x: f32, y: f32, button: cc::Mousebutton) {
    MOUSE_DOWN.store(false, Ordering::Relaxed);
    println!("mouse released: x={x}, y={y}, button={button:?}");
}

fn on_mouse_moved(x: f32, y: f32) {
    let _ = (x, y);
}
