use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;

pub trait FrameSuccesor {
    fn run_one_frame(&mut self);
}

pub fn run_request_animation_frame_loop(
    frame: Rc<RefCell<dyn FrameSuccesor>>,
) -> Rc<RefCell<Option<Closure<dyn FnMut()>>>> {
    // We use Rc<RefCell<None>> trick for recursive calling of request_animation_frame.
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        frame.as_ref().borrow_mut().run_one_frame();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));
    request_animation_frame(g.borrow().as_ref().unwrap());
    g
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
