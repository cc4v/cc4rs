use cc4rs::*;
use std::sync::{LazyLock, Mutex};

static IMAGE: LazyLock<Mutex<Option<Image>>> = LazyLock::new(|| Mutex::new(None));

fn main() {
    on_init!(setup);
    on_exit!(cleanup);
    run(draw);
}

fn setup() {
    match load_image("examples/assets/sample.png") {
        Ok(image) => *IMAGE.lock().unwrap() = Some(image),
        Err(error) => eprintln!("failed to load image: {error}"),
    }
}

fn draw() {
    set_color(colors::white());
    if let Some(texture) = IMAGE.lock().unwrap().as_ref() {
        image(texture, 0.0, 0.0);
        image_with_size(texture, 180.0, 0.0, 100.0, 100.0);
    } else {
        set_color(colors::red());
        text("Image not found", 20.0, 20.0);
    }
}

fn cleanup() {
    if let Some(mut image) = IMAGE.lock().unwrap().take() {
        delete_image(&mut image);
    }
}
