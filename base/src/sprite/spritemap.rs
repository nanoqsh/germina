use {
    crate::sprite::{pack::Packed, Rect},
    image::{GenericImage, GenericImageView, RgbaImage as Image, SubImage},
};

pub struct SpriteMap {
    pub rects: Vec<Rect>,
    pub image: Image,
}

impl SpriteMap {
    pub fn new(images: &[SubImage<&Image>]) -> Self {
        use std::iter::zip;

        let mut rects = vec![Rect::default(); images.len()];

        let sizes = images.iter().map(|image| {
            (
                u16::try_from(image.width()).expect("too large image"),
                u16::try_from(image.height()).expect("too large image"),
            )
        });

        let Packed { frames, side } = Packed::pack(zip(0.., sizes));
        let mut image = {
            let side = u32::from(side);
            Image::new(side, side)
        };

        for (index, rect) in frames {
            rects[index] = rect;

            let view = *images[index];
            let Rect { pos: (x, y), .. } = rect;
            image
                .copy_from(&view, u32::from(x), u32::from(y))
                .expect("copy");
        }

        Self { rects, image }
    }
}
