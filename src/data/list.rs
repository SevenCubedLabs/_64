use super::page::Page;

pub struct List<Item: Sized> {
    data: Page<Item>,
    len: usize,
}

impl<Item: Sized> List<Item> {
    pub fn new() -> Self {
        Self {
            data: Page::new(1),
            len: 0,
        }
    }

    pub fn push(&mut self, item: Item) {
        if self.len >= self.data.capacity() {
            self.data.resize(2 * self.data.capacity());
        }

        self.data[self.len] = item;
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
