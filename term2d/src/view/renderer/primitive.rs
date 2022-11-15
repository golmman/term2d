use std::{cell::RefCell, rc::Rc};

use crate::view::canvas::Canvas;

pub struct PrimitiveRenderer<T: Canvas> {
    canvas: Option<Rc<RefCell<T>>>,
}

impl<T: Canvas> PrimitiveRenderer<T> {
    pub fn new() -> Self {
        Self { canvas: None }
    }

    pub fn init(&mut self, canvas: &Rc<RefCell<T>>) {
        self.canvas = Some(canvas.clone());
    }
}
