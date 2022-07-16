use crate::sprite::Rect;
use std::{cmp::Eq, collections::HashMap, hash::Hash};

pub(super) struct Packed<K> {
    pub frames: HashMap<K, Rect>,
    pub side: u16,
}

impl<K> Packed<K> {
    pub fn pack<F>(frames: F) -> Self
    where
        F: IntoIterator<Item = (K, (u16, u16))>,
        K: Eq + Hash + Clone,
    {
        let mut frames: Vec<_> = frames
            .into_iter()
            .map(|(key, size)| Frame { size, key })
            .collect();

        let (frames, side) = pack_frames(&mut frames);
        Self { frames, side }
    }
}

fn pack_frames<K>(frames: &mut [Frame<K>]) -> (HashMap<K, Rect>, u16)
where
    K: Eq + Hash + Clone,
{
    use std::cmp::Reverse;

    frames.sort_by_key(|frame| Reverse((frame.size.1, frame.size.0)));

    let mut side = calc_side(frames);
    loop {
        match try_pack(frames, side) {
            Some(rects) => return (rects, side),
            None => side <<= 1,
        }
    }
}

struct Frame<K> {
    size: (u16, u16),
    key: K,
}

impl<K> Frame<K> {
    fn width(&self) -> u16 {
        self.size.0
    }

    fn height(&self) -> u16 {
        self.size.1
    }
}

fn calc_side<K>(frames: &[Frame<K>]) -> u16 {
    use std::cmp::max;

    let max_size = frames
        .iter()
        .map(|frame| max(frame.width(), frame.height()))
        .max()
        .unwrap_or_default();

    let area: u32 = frames
        .iter()
        .map(|frame| u32::from(frame.width()) * u32::from(frame.height()))
        .sum();

    let area_sqrt = (area as f32).sqrt().ceil() as u16;
    let side = max(max_size, area_sqrt);
    side.next_power_of_two()
}

fn try_pack<K>(frames: &[Frame<K>], side: u16) -> Option<HashMap<K, Rect>>
where
    K: Eq + Hash + Clone,
{
    let mut x = 0;
    let mut y = 0;
    let mut max_height = 0;

    frames
        .iter()
        .map(|frame| {
            let width = frame.width();
            let height = frame.height();
            max_height = max_height.max(height);

            if x + width > side {
                x = 0;
                y += max_height;
                max_height = 0;
            }

            if y + height > side {
                return None;
            }

            let res = (
                frame.key.clone(),
                Rect {
                    pos: (x, y),
                    size: (width, height),
                },
            );
            x += width;
            Some(res)
        })
        .collect()
}
