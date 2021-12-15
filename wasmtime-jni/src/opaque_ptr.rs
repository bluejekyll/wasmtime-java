use std::any;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use jni::sys::jlong;
use log::{debug, trace};

/// List of Opaque types that we support for passing to and from Java
pub(crate) trait Opaqueable {}

impl Opaqueable for wasmtime::Engine {}
impl Opaqueable for wasmtime::Func {}
impl Opaqueable for wasmtime::Instance {}
impl<T> Opaqueable for wasmtime::Linker<T> {}
impl Opaqueable for wasmtime::Module {}
impl<T> Opaqueable for wasmtime::Store<T> {}

// TODO: add methods to extract from a passed in Object to have better ownership semantics in Java.
/// This borrows the pointer stored at jlong, not taking ownership
///
/// This should only be used with [`make_opaque`]
#[repr(transparent)]
pub struct OpaquePtr<'a, T> {
    ptr: jlong,
    ty: PhantomData<&'a T>,
}

impl<'a, T> OpaquePtr<'a, T> {
    pub(crate) fn from(val: T) -> Self
    where
        T: Sized + Opaqueable,
    {
        let ptr: jlong = Box::into_raw(Box::new(val)) as jlong;

        let this = Self {
            ptr,
            ty: PhantomData,
        };

        debug!("{:?}::from", this);

        this
    }

    #[track_caller]
    pub fn as_ref(&self) -> &'a T {
        trace!("{:?}::as_ref", self);
        assert_ne!(
            self.ptr,
            0,
            "cannot deref null for &{}",
            any::type_name::<T>()
        );
        let obj = self.ptr as *const T;

        unsafe { &*obj }
    }

    #[track_caller]
    pub fn as_mut(&mut self) -> &mut T {
        trace!("{:?}::as_mut", self);
        assert_ne!(
            self.ptr,
            0,
            "cannot deref null for &{}",
            any::type_name::<T>()
        );
        let obj = self.ptr as *mut T;

        unsafe { &mut *obj }
    }

    /// This takes ownership of the pointer stored at jlong.
    ///
    /// It is undefined behavior to reference the ptr in any other context after this.
    #[track_caller]
    pub fn take(self) -> Box<T> {
        trace!("{:?}::take", self);
        assert_ne!(
            self.ptr,
            0,
            "cannot deref null for &{}",
            any::type_name::<T>()
        );
        let obj = self.ptr as *mut T;

        unsafe { Box::from_raw(obj) }
    }

    /// Take ownership of a Rust type and return an opaque pointer as a jlong for future usage
    pub fn make_opaque(self) -> jlong {
        self.ptr
    }

    /// Returns true if the backing ptr is == 0
    #[allow(unused)]
    pub fn is_null(&self) -> bool {
        self.ptr == 0
    }
}

impl<'a, T> Deref for OpaquePtr<'a, T> {
    type Target = T;

    #[track_caller]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T> DerefMut for OpaquePtr<'a, T> {
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<'a, T> fmt::Debug for OpaquePtr<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "OpaquePtr<{}>({})", std::any::type_name::<T>(), self.ptr)
    }
}
