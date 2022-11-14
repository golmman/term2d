use std::{cell::RefCell, rc::Rc};

use crate::renderer::{half_block_renderer::HalfblockCanvas, Renderer};

pub struct PrimitiveRenderer<T: Renderer> {
    canvas: Option<Rc<RefCell<T>>>,
}

impl<T: Renderer> PrimitiveRenderer<T> {
    pub fn new() -> Self {
        Self { canvas: None }
    }

    pub fn init(&mut self, canvas: &Rc<RefCell<T>>) {
        self.canvas = Some(canvas.clone());
    }
}
