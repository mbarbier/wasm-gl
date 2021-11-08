use game::example1;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use weblog::console_log;

mod core;
mod game;


fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame'");
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console_log!("Starting wasm webgl2");

    let window = web_sys::window().expect("No global window object");

    let (mut game_resize, mut game_update) = example1()?;
    game_resize(
        window.inner_width().unwrap().as_f64().unwrap(),
        window.inner_height().unwrap().as_f64().unwrap(),
    );

    let windowc = window.clone();
    let closure = Closure::wrap(Box::new(move |_event: web_sys::UiEvent| {
        game_resize(
            windowc.inner_width().unwrap().as_f64().unwrap(),
            windowc.inner_height().unwrap().as_f64().unwrap(),
        );
    }) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_event_listener("resize", closure.as_ref().unchecked_ref())?;
    closure.forget();

    let f = std::rc::Rc::new(std::cell::RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game_update();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
