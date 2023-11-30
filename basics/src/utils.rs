pub fn get_window_size() -> (f32, f32) {
    let window = web_sys::window().unwrap();
    (
        window.inner_width().unwrap().as_f64().unwrap() as f32,
        window.inner_height().unwrap().as_f64().unwrap() as f32,
    )
}
