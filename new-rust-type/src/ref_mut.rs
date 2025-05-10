use std::cell::UnsafeCell;
/// RefMut can take any type
pub struct RefMut<T> {
    value: UnsafeCell<T>
}
// you can update the value with a mutable reference.
//impl<T> !Sync for RefMut<T> {}


impl<T> RefMut<T> {
    pub fn new(t:T) -> Self {
        RefMut {
            value: UnsafeCell::new(t)
        }
    }
    pub fn get_data(&self) -> T 
    where T: Copy
    {
        unsafe { *self.value.get() }
    }

    pub fn get_ref(&self) -> &T {
        /// returning the reference to the type. 
        unsafe { &*self.value.get() }
    }

    pub fn get_ptr(&self) -> *mut T {
        /// returning the raw mutable pointer. 
        self.value.get()
    }

    pub fn set_data(&self, value: T) {
        //self.value.replace(value);
        // so we get a raw mutable pointer
        unsafe { *self.value.get() = value };
    }
}