use {crate::chunk::layout::Layout, std::marker::PhantomData};

pub struct Data<T, L, const N: usize> {
    inner: [T; N],
    layout: PhantomData<L>,
}

impl<T, L, const N: usize> Data<T, L, N>
where
    L: Layout,
{
    pub fn new(val: T) -> Self
    where
        T: Copy,
    {
        Self {
            inner: [val; N],
            layout: PhantomData,
        }
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds of chunk is undefined behavior.
    pub unsafe fn get_unchecked(&self, point: (u32, u32, u32)) -> &T {
        let index = L::to_index(point) as usize;
        debug_assert!(index < N);
        self.inner.get_unchecked(index)
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds of chunk is undefined behavior.
    pub unsafe fn get_unchecked_mut(&mut self, point: (u32, u32, u32)) -> &mut T {
        let index = L::to_index(point) as usize;
        debug_assert!(index < N);
        self.inner.get_unchecked_mut(index)
    }
}
