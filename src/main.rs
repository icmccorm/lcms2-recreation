use foreign_types::foreign_type;
use foreign_types::ForeignType;
use foreign_types::ForeignTypeRef;

#[repr(C)]
pub struct ExampleInner {
    inner: u32,
}

fn destructor(ptr: *mut ExampleInner) {
    unsafe {
        drop(Box::from_raw(ptr));
    }
}

foreign_type! {
    /// This represents owned Multi Localized Unicode type. Most methods are implemented on `MLURef`.
    /// This is a borrwed Multi Localized Unicode type. It holds Unicode strings associated with `Locale`.
    pub unsafe type Example {
        type CType = ExampleInner;
        fn drop = destructor;
    }
}

impl Example {
    /// Allocates an empty multilocalized unicode object.
    #[track_caller]
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        unsafe { Example::from_ptr(Box::leak(Box::new(ExampleInner { inner: 0 }))) }
    }
}

impl ExampleRef {
    fn invalid_mutation(&mut self) {
        let raw_ptr: *mut ExampleInner = self.as_ptr();
        unsafe {
            (*raw_ptr).inner = 1;
        }
    }
}

fn main() {
    let mut ex = Example::new();
    ex.invalid_mutation();
}