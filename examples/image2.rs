use cc4rs::*;
use global_var::new_global_var;

new_global_var!(Option<Image>, None);

fn main() {
    on_init!(setup);
    on_exit!(cleanup);
    run(draw);
}

fn setup() {
    unsafe {
        match load_image("examples/assets/sample.png") {
            Ok(image) => *get_mut_global_var() = Some(image),
            Err(error) => eprintln!("failed to load image: {error}"),
        }
    }
}

fn draw() {
    set_color(colors::white());
    unsafe {
        if let Some(texture) = get_mut_global_var().as_ref() {
            image(texture, 0.0, 0.0);
            image_with_size(texture, 180.0, 0.0, 100.0, 100.0);
        } else {
            set_color(colors::red());
            text("Image not found", 20.0, 20.0);
        }
    }
}

fn cleanup() {
    unsafe {
        if let Some(mut image) = get_mut_global_var().take() {
            delete_image(&mut image);
        }
    }
}
