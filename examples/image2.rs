use cc4rs::*;

fn main() {
    let state = Box::into_raw(Box::new(None::<Image>)) as RawPtr;
    on_init!(setup, with_data);
    on_exit!(cleanup, with_data);
    run_with_data(draw, state);
}

fn setup(user_data: RawPtr) {
    unsafe {
        let image = &mut *(user_data as *mut Option<Image>);
        *image = load_image("examples/assets/sample.png").ok();
    }
}

fn draw(user_data: RawPtr) {
    set_color(colors::white());
    unsafe {
        let state = &*(user_data as *const Option<Image>);
        if let Some(texture) = state {
            image(texture, 0.0, 0.0);
        }
    }
}

fn cleanup(user_data: RawPtr) {
    unsafe {
        let state = Box::from_raw(user_data as *mut Option<Image>);
        if let Some(mut image) = *state {
            delete_image(&mut image);
        }
    }
}
