use std::{any, mem};

use jni::sys::jlong;
use jni::JNIEnv;
use log::debug;

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
