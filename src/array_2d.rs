use std::fmt::Debug;

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
    pub fn from_flat(width: usize, iter: impl Iterator<Item = T>) -> Self {
        let buf = Vec::from_iter(iter);
        Self {
            width,
            height: buf.len() / width,
            buf,
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
    pub fn flat_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.buf.iter_mut()
    }
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.buf.chunks_exact(self.width)
    }
    pub fn cols(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.width).map(|x| {
            let mut col = Vec::with_capacity(self.height);
            for y in 0..self.height {
                col.push(self.get(x, y));
            }
            col
        })
    }
}

impl<T: Debug> Debug for Array2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f)?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{:?} ", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
