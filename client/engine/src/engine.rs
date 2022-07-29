use {
    crate::{state::State, view::View},
    render::ClientRender as Render,
    std::time::Instant,
};

pub struct Engine {
    state: State,
    view: View,
    time: Time,
}

impl Engine {
    pub fn new(render: Render) -> Self {
        Self {
            state: State::new(),
            view: View::new(render),
            time: Time::new(),
        }
    }

    pub fn update(&mut self) {
        let delta = self.time.delta();
        self.state.update(delta);
        self.view.render_state(&self.state);
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        self.view.resize(size);
    }
}

struct Time {
    last: Instant,
}

impl Time {
    fn new() -> Self {
        Self {
            last: Instant::now(),
        }
    }

    fn delta(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last);
        self.last = now;
        delta.as_secs_f32()
    }
}
