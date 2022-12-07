use core::ffi::c_void;
use core::mem::size_of;

extern "C" {
    fn malloc(_: usize) -> *mut c_void;
    fn memcpy(_: *const c_void, _: *mut c_void, _: usize) -> *mut c_void;
    fn free(_: *mut c_void);
}

#[derive(Debug, Clone)]
pub struct Vec<Item: Sized> {
    data: *mut Item,
    len: usize,
    capacity: usize,
}

impl<Item: Sized> Vec<Item> {
    const ITEM_SIZE: usize = size_of::<Item>();

    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Vec {
            data: unsafe { malloc((capacity * Self::ITEM_SIZE) as _) as _ },
            len: 0,
            capacity,
        }
    }

    pub fn push(&mut self, item: Item) {
        if self.len >= self.capacity {
            self.resize(2 * self.capacity);
        }

        unsafe {
            self.data.add(self.len).write(item);
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
                (self.len * Self::ITEM_SIZE) as _,
            );
            free(self.data as _);
            new_data
        };

        self.capacity = capacity;
    }
}

impl<Item> From<&[Item]> for Vec<Item> {
    fn from(items: &[Item]) -> Self {
        let mut list = Vec::with_capacity(items.len());
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

impl<Item> core::ops::Deref for Vec<Item> {
    type Target = [Item];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.data, self.len) }
    }
}

impl<Item> core::ops::Index<usize> for Vec<Item> {
    type Output = Item;

    fn index(&self, idx: usize) -> &Self::Output {
        unsafe { &*self.data.add(idx) }
    }
}

impl<Item> core::ops::IndexMut<usize> for Vec<Item> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        unsafe { &mut *self.data.add(idx) }
    }
}

impl<Item> core::iter::FromIterator<Item> for Vec<Item> {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let mut list = Self::with_capacity(1);
        for x in iter {
            list.push(x);
        }

        list
    }
}

impl<Item> Drop for Vec<Item> {
    fn drop(&mut self) {
        unsafe {
            free(self.data as _);
        }
    }
}
