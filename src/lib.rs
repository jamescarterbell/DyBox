use std::alloc::{dealloc, Layout};
use std::ops::{Deref, DerefMut};

pub struct DyBox<T>{
    data: *mut T,
    drop_fn: unsafe extern fn(*mut u8, Layout),
    layout: std::alloc::Layout,
}

impl<T> DyBox<T>{
    fn new(data: T) -> Self{
        Self{
            data: unsafe{Box::into_raw(Box::new(data))},
            drop_fn: drop_ptr,
            layout: Layout::new::<T>(),
        }
    }
}

impl<T> Drop for DyBox<T>{
    fn drop(&mut self){;
        unsafe{drop_ptr(self.data as *mut u8, self.layout)}
    }
}

impl<T> Deref for DyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        unsafe{self.data.as_ref().unwrap()}
    }
}

impl<T> DerefMut for DyBox<T>{
    fn deref_mut(&mut self) -> &mut Self::Target{
        unsafe{self.data.as_mut().unwrap()}
    }
}

#[no_mangle]
unsafe extern fn drop_ptr(ptr: *mut u8, layout: Layout){
    dealloc(ptr, layout)
}