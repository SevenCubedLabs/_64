use crate::sys::{free, malloc, memcpy};
use core::mem::size_of;

pub struct Page<Item: Sized> {
    data: *mut Item,
    capacity: usize,
}

impl<Item: Sized> Page<Item> {
    const ITEM_SIZE: usize = size_of::<Item>();

    pub fn new(capacity: usize) -> Self {
        Page {
            data: unsafe { malloc((capacity * Self::ITEM_SIZE) as _) as _ },
            capacity,
        }
    }

    pub fn resize(&mut self, capacity: usize) {
        self.data = unsafe {
            let new_data = malloc((capacity * Self::ITEM_SIZE) as _) as *mut Item;
            memcpy(
                new_data as _,
                self.data as _,
                (self.capacity * Self::ITEM_SIZE) as _,
            );
            new_data
        };

        self.capacity = capacity;
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<Item: Sized> From<&[Item]> for Page<Item> {
    fn from(items: &[Item]) -> Self {
        let page = Page::new(items.len());
        unsafe {
            memcpy(
                page.data as _,
                items.as_ptr() as _,
                (items.len() * Self::ITEM_SIZE) as _,
            );
        }
        page
    }
}

impl<Item: Sized> core::ops::Index<usize> for Page<Item> {
    type Output = Item;

    fn index(&self, idx: usize) -> &Self::Output {
        unsafe { &*self.data.add(idx * Self::ITEM_SIZE) }
    }
}

impl<Item: Sized> core::ops::IndexMut<usize> for Page<Item> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        unsafe { &mut *self.data.add(idx * Self::ITEM_SIZE) }
    }
}

impl<Item: Sized> Drop for Page<Item> {
    fn drop(&mut self) {
        unsafe {
            free(self.data as _);
        }
    }
}

#[test]
fn new() {
    let page = Page::<f32>::new(5);

    assert!(!page.data.is_null());
    assert_eq!(page.n, 5);
}

#[test]
fn index() {
    let page: Page<u8> = [0, 1, 2].as_slice().into();

    assert_eq!(page[0], 0);
    assert_eq!(page[1], 1);
    assert_eq!(page[2], 2);
}

#[test]
fn resize() {
    let mut page: Page<u8> = [0, 1, 2].as_slice().into();

    //Test new size
    assert_eq!(page.n, 3);
    page.resize(5);
    assert_eq!(page.n, 5);

    //Preserve original contents
    assert_eq!(page[0], 0);
    assert_eq!(page[1], 1);
    assert_eq!(page[2], 2);
}
