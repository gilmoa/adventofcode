#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T> Grid<T> {
    fn new(data: Vec<T>, width: usize) -> Self {
        Self {
            height: data.len() / width,
            data,
            width,
        }
    }

    fn row(&self, i: usize) -> Option<&[T]> {
        self.data.get(i * self.width..i * self.width + self.width)
    }

    fn col(&self, i: usize) -> Option<&[T]> {}
}
