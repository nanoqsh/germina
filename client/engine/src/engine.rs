use crate::{state::State, view::View};
use client_core::render::Render;

pub struct Engine<R>
where
    R: Render,
{
    state: State,
    view: View<R>,
}

impl<R> Engine<R>
where
    R: Render,
{
    pub fn new(render: R) -> Self {
        Self {
            state: State::new(),
            view: View::new(render),
        }
    }

    pub fn resize_view(&mut self, size: (u32, u32)) {
        self.view.resize(size);
    }

    pub fn draw_view(&mut self) {
        self.view.render_state(&self.state);
    }
}
