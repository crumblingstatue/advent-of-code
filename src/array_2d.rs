pub struct Array2D<T> {
    buf: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Array2D<T> {
    pub fn new_filled(width: usize, height: usize, fill: T) -> Self
    where
        T: Copy,
    {
        Self {
            width,
            height,
            buf: vec![fill; width * height],
        }
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.buf[y * self.width + x]
    }
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.buf[y * self.width + x]
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn flat_iter(&self) -> impl Iterator<Item = &T> {
        self.buf.iter()
    }
}
