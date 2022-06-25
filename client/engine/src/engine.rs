use crate::{state::State, view::View};
use render::ClientRender as Render;

pub struct Engine {
    state: State,
    view: View,
}

impl Engine {
    pub fn new(render: Render) -> Self {
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
