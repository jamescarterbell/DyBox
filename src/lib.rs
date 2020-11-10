#![feature(unsize)]
#![feature(coerce_unsized)]

use std::alloc::{dealloc, Layout};
use std::ops::{Deref, DerefMut};
use std::marker::Unsize;
use std::ops::CoerceUnsized;

pub struct DyBox<T>
    where T: ?Sized{
    data: *mut T,
    drop_fn: unsafe extern fn(*mut u8, Layout),
    layout: std::alloc::Layout,
}

impl<T> DyBox<T>{
    pub fn new(data: T) -> Self{
        let data = unsafe{Box::into_raw(Box::new(data))};
        let layout = Layout::for_value(&data);
        Self{
            data,
            drop_fn: drop_ptr,
            layout,
        }
    }
}

impl<T> Drop for DyBox<T>
    where T: ?Sized{
    fn drop(&mut self){
        unsafe{(self.drop_fn)(self.data as *mut u8, self.layout)}
    }
}

impl<T> Deref for DyBox<T>
    where T: ?Sized{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        unsafe{self.data.as_ref().unwrap()}
    }
}

impl<T> DerefMut for DyBox<T>
    where T: ?Sized{
    fn deref_mut(&mut self) -> &mut Self::Target{
        unsafe{self.data.as_mut().unwrap()}
    }
}

impl<T, U> CoerceUnsized<DyBox<U>> for DyBox<T>
    where T: Unsize<U>,
          U: ?Sized{

          }

#[no_mangle]
unsafe extern fn drop_ptr(ptr: *mut u8, layout: Layout){
    dealloc(ptr, layout)
}

#[cfg(test)]
mod tests{

    use super::DyBox;

    #[test]
    fn make_free(){
        let thingy = DyBox::new(5);
        drop(thingy);
    }

    trait Dab{}
    impl Dab for u32{}

    #[test]
    fn coerce_free(){
        let thingy = DyBox::new(5 as u32);
        let thingy = thingy as DyBox<dyn Dab>;
        drop(thingy);
    }
}