use core::mem::size_of;
use underscore_sys::*;

pub struct List<Item: Sized> {
    data: *mut Item,
    len: usize,
    capacity: usize,
}

impl<Item: Sized> List<Item> {
    const ITEM_SIZE: usize = size_of::<Item>();

    pub fn new(capacity: usize) -> Self {
        List {
            data: unsafe { malloc((capacity * Self::ITEM_SIZE) as _) as _ },
            len: 0,
            capacity,
        }
    }

    pub fn push(&mut self, item: Item) {
        if self.len + 1 >= self.capacity {
            self.resize(2 * self.capacity);
        }

        unsafe {
            *self.data.add(self.len) = item;
        }
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn resize(&mut self, capacity: usize) {
        self.data = unsafe {
            let new_data = malloc((capacity * Self::ITEM_SIZE) as _) as *mut Item;
            memcpy(
                new_data as _,
                self.data as _,
                (self.len * size_of::<Item>()) as _,
            );
            new_data
        };

        self.capacity = capacity;
    }
}

impl<Item: Sized> From<&[Item]> for List<Item> {
    fn from(items: &[Item]) -> Self {
        let mut list = List::new(items.len());
        unsafe {
            memcpy(
                list.data as _,
                items.as_ptr() as _,
                (items.len() * Self::ITEM_SIZE) as _,
            );
        }

        list.len = items.len();
        list
    }
}

impl<Item: Sized> core::ops::Deref for List<Item> {
    type Target = [Item];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.data, self.len) }
    }
}

impl<Item: Sized> core::ops::Index<usize> for List<Item> {
    type Output = Item;

    fn index(&self, idx: usize) -> &Self::Output {
        unsafe { &*self.data.add(idx) }
    }
}

impl<Item: Sized> core::ops::IndexMut<usize> for List<Item> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        unsafe { &mut *self.data.add(idx) }
    }
}

impl<Item: Sized> Drop for List<Item> {
    fn drop(&mut self) {
        unsafe {
            free(self.data as _);
        }
    }
}

#[test]
fn new() {
    let list = List::<f32>::new(5);

    assert!(!list.data.is_null());
    assert_eq!(list.capacity, 5);
}

#[test]
fn index() {
    let list: List<u8> = [0, 1, 2].as_slice().into();

    assert_eq!(list[0], 0);
    assert_eq!(list[1], 1);
    assert_eq!(list[2], 2);
}

#[test]
fn resize() {
    let mut list: List<u8> = [0, 1, 2].as_slice().into();

    //Test new size
    assert_eq!(list.capacity, 3);
    list.resize(5);
    assert_eq!(list.capacity, 5);
    list.resize(100);
    assert_eq!(list.capacity, 100);

    //Preserve original contents
    assert_eq!(list[0], 0);
    assert_eq!(list[1], 1);
    assert_eq!(list[2], 2);
    assert_eq!(list.len(), 3);
}

#[test]
fn push() {
    let mut list = List::new(1000);

    for x in 0..1000 {
        list.push(x);
    }

    assert_eq!(list.len(), 1000);

    assert_eq!(list[0], 0);
    assert_eq!(list[999], 999);
}

#[test]
fn deref() {
    let list: List<u8> = [0, 1, 2].as_slice().into();

    let slice: &[u8] = &list;
    assert_eq!(slice.len(), list.len());

    //Check contents of new slice
    assert_eq!(slice[0], 0);
    assert_eq!(slice[1], 1);
    assert_eq!(slice[2], 2);
}
