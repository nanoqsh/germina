use std::{marker::PhantomData, num::NonZeroU32};

pub type Size = (NonZeroU32, NonZeroU32);

pub trait Render: Sized {
    type Mesh: Mesh;
    type Texture: Texture;

    fn resize(&mut self, size: Size);
    fn start_frame(&mut self) -> Frame<Self>;
    fn make_mesh(&mut self) -> Self::Mesh;
    fn make_texture(&mut self) -> Self::Texture;
}

pub trait SetPass<P> {
    fn set_pass(&mut self);
}

pub trait Passes: SetPass<pass::Solid> + SetPass<pass::Color> {}
impl<T> Passes for T where T: SetPass<pass::Solid> + SetPass<pass::Color> {}

pub mod pass {
    pub struct Solid;
    pub struct Color;
}

pub struct Frame<'a, R>(&'a mut R);

impl<'a, R> Frame<'a, R> {
    pub fn new(render: &'a mut R) -> Self {
        Self(render)
    }

    pub fn pass<P>(&mut self, _: P) -> Pass<'_, R, P>
    where
        R: Render + SetPass<P>,
    {
        self.0.set_pass();
        Pass::from_render(&mut self.0)
    }
}

pub struct Pass<'a, R, P> {
    render: &'a mut R,
    pass: PhantomData<P>,
}

impl<'a, R, P> Pass<'a, R, P> {
    fn from_render(render: &'a mut R) -> Self
    where
        R: Render,
    {
        Self {
            render,
            pass: PhantomData,
        }
    }
}

impl<R> Pass<'_, R, pass::Solid> {
    pub fn draw_mesh<M>(&mut self, mesh: &M)
    where
        M: Mesh<Render = R>,
    {
        mesh.draw(&mut self.render);
    }

    pub fn bind_texture<T>(&mut self, texture: &T)
    where
        T: Texture<Render = R>,
    {
        texture.bind(&mut self.render);
    }

    pub fn set_model(&mut self, _: &()) {
        //
    }
}

pub trait Mesh {
    type Render: Render;

    fn draw(&self, render: &mut Self::Render);
}

pub trait Texture {
    type Render: Render;

    fn bind(&self, render: &mut Self::Render);
}
