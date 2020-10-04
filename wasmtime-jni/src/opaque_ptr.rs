use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::{any, mem};

use jni::sys::jlong;
use jni::JNIEnv;
use log::debug;

#[repr(transparent)]
pub struct OpaquePtr<'a, T> {
    ptr: jlong,
    ty: PhantomData<&'a T>,
}

impl<'a, T> OpaquePtr<'a, T> {
    pub fn as_ref(&self) -> &'a T {
        debug!("opaque_ptr({}) to &{}", self.ptr, any::type_name::<T>());
        assert_ne!(self.ptr, 0, "cannot deref null");
        let obj = self.ptr as *const T;

        unsafe { mem::transmute(obj) }
    }

    pub fn as_mut(&mut self) -> &mut T {
        debug!("opaque_ptr({}) to &{}", self.ptr, any::type_name::<T>());
        assert_ne!(self.ptr, 0, "cannot deref null");
        let obj = self.ptr as *const T;

        unsafe { mem::transmute(obj) }
    }

    pub fn take(self) -> Box<T> {
        debug!("opaque_ptr({}) to Box<{}>", self.ptr, any::type_name::<T>());
        assert_ne!(self.ptr, 0, "cannot deref null");
        let obj = self.ptr as *mut T;

        unsafe { Box::from_raw(obj) }
    }

    pub fn make_opaque(self) -> jlong {
        self.ptr
    }
}

impl<'a, T> Deref for OpaquePtr<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T> DerefMut for OpaquePtr<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> From<T> for OpaquePtr<'static, T>
where
    T: Sized,
{
    fn from(val: T) -> Self {
        let ptr: jlong = Box::into_raw(Box::new(val)) as jlong;
        debug!("opaque_ptr({}) from {}", ptr, any::type_name::<T>());

        Self {
            ptr,
            ty: PhantomData,
        }
    }
}

/// This borrows the pointer stored at jlong, not taking ownership
///
/// This should only be used with [`to_jlong`]
#[track_caller]
pub unsafe fn ref_from_jlong<'j, T: Sized>(_jni_env: &JNIEnv<'j>, ptr: jlong) -> &'j T {
    debug!("opaque_ptr({}) to &{}", ptr, any::type_name::<T>());
    assert_ne!(ptr, 0, "cannot deref null");
    let obj = ptr as *const T;

    mem::transmute(obj)
}

/// This takes ownership of the pointer stored at jlong.
///
/// It is undefined behavior to reference the ptr in any other context after this.
#[track_caller]
pub unsafe fn box_from_jlong<T: Sized>(ptr: jlong) -> Box<T> {
    debug!("opaque_ptr({}) to Box<{}>", ptr, any::type_name::<T>());
    assert_ne!(ptr, 0, "cannot deref null");
    let obj = ptr as *mut T;

    Box::from_raw(obj)
}

/// Take ownership of a Rust type and return an opaque pointer as a jlong for future usage
///
/// This should only be used with [`box_from_jlong`] and [`ref_from_long`]
#[track_caller]
pub fn to_jlong<T: Sized>(t: T) -> jlong {
    let ptr: jlong = Box::into_raw(Box::new(t)) as jlong;
    debug!("opaque_ptr({}) from {}", ptr, any::type_name::<T>());

    ptr
}
