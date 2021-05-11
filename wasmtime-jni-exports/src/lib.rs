use std::borrow::Cow;
use std::ops::Deref;
use std::slice;
use std::{
    alloc::{self, Layout},
    marker::PhantomData,
};

pub const MEMORY_EXPORT: &str = "memory";
pub const ALLOC_EXPORT: &str = "__alloc_bytes";
pub const DEALLOC_EXPORT: &str = "__dealloc_bytes";

/// Allocates size in bytes of `memory`, offset to area returned.
///
/// # Returns
///
/// Offset from start of `memory` export in WASM to the region, or 0 if unable to allocate.
///
/// # Safety
///
/// This will allocate a byte array in the WASM module. To refer to this memory externally, there is an
///  exported memory section. These bytes are only valid for the life of the Store or until the Memory is
///  resized.
#[no_mangle]
pub unsafe extern "C" fn __alloc_bytes(size: u32) -> i32 {
    let layout = Layout::array::<u8>(size as usize).expect("u8 should definitely have a layout");
    let ptr = alloc::alloc(layout) as i32;

    debug_assert_ne!(0, ptr);
    // useful for debugging
    //println!("allocated {} at {}", size, ptr);
    ptr
}

/// Frees ptr from `memory` in WASM
///
/// # Safety
///
/// Must be a pointer to data allocated with the __alloc_bytes
#[no_mangle]
pub unsafe extern "C" fn __dealloc_bytes(ptr: u32, size: u32) {
    let layout = Layout::array::<u8>(size as usize).expect("u8 should definitely have a layout");
    alloc::dealloc(ptr as *mut u8, layout);
}

/// Data that was allocated inside a WASM module
pub trait WasmAllocated: Sized {
    /// Return the WASM offset pointer
    fn ptr(&self) -> i32;

    /// Return the WASM offset pointer
    fn len(&self) -> i32;
}

/// A WasmSlice is an offset into the local `memory` of the WASM module instance.
///
/// It is only valid in the context of a `memory` contiguous region and a module's associated `Store`
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WasmSlice {
    // FIXME: need Owned and Borrowed variants of this...
    ptr: i32,
    len: i32,
}

impl WasmSlice {
    /// Danger, danger, you probable want `owned` or `borrowed`. This will produce something that is neither.
    pub unsafe fn new(ptr: i32, len: i32) -> Self {
        Self { ptr, len }
    }

    /// Create an owned reference to a byte slice to the memory at `ptr` with a length of `len`.
    ///
    /// This will be dropped and freed at the end of usaged
    ///
    /// # Safety
    ///
    /// The `ptr` must be a valid offset in the WASM module for the slice of the specified length. It must abide by the same rules as a `&[u8]`.
    pub unsafe fn owned(ptr: i32, len: i32) -> Owned<Self> {
        Owned(Self { ptr, len })
    }

    /// Create an borrowed reference to a byte slice to the memory at `ptr` with a length of `len`.
    ///
    /// This will *not* be dropped or freed at the end of usaged.
    ///
    /// # Safety
    ///
    /// The `ptr` must be a valid offset in the WASM module for the slice of the specified length. It must abide by the same rules as a `&[u8]`.
    pub unsafe fn borrowed(ptr: &i32, len: i32) -> Borrowed<'_, Self> {
        Borrowed {
            val: WasmSlice { ptr: *ptr, len },
            ghost: PhantomData,
        }
    }

    /// # Safety
    /// This relies on the ptr and len being accurate for the current memory environment. Inside a WASM runtime for example.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr = self.ptr as *const u8;
            slice::from_raw_parts(ptr, self.len as usize)
        }
    }
}

impl WasmAllocated for WasmSlice {
    /// Return the WASM offset pointer
    fn ptr(&self) -> i32 {
        self.ptr
    }

    /// Return the WASM offset pointer
    fn len(&self) -> i32 {
        self.len
    }
}

impl From<Vec<u8>> for Owned<WasmSlice> {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        let boxed_slice = bytes.into_boxed_slice();
        Owned::<WasmSlice>::from(boxed_slice)
    }
}

// FIXME: this either needs a Drop impl or should take a slice of bytes and have an associated lifetime instead...
impl From<Box<[u8]>> for Owned<WasmSlice> {
    #[inline]
    fn from(bytes: Box<[u8]>) -> Self {
        let len = bytes.len() as i32;
        let ptr = Box::into_raw(bytes) as *mut u8 as i32;

        // helpful for debugging
        // println!("storing Box<[u8]> at {} len {}", ptr, len);

        Owned(WasmSlice { ptr, len })
    }
}

/// A WasmString is an offset into the local `memory` of the WASM module instance.
///
/// It is only valid in the context of a `memory` contiguous region and a module's associated `Store`
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct WasmString(WasmSlice);

impl WasmString {
    /// Create an owned reference to a byte slice to the memory at `ptr` with a length of `len`.
    ///
    /// This will be dropped and freed at the end of usaged
    ///
    /// # Safety
    ///
    /// The `ptr` must be a valid offset in the WASM module for the slice of the specified length. It must abide by the same rules as a `&[u8]`.
    pub unsafe fn owned(ptr: i32, len: i32) -> Owned<Self> {
        Owned(Self(WasmSlice { ptr, len }))
    }

    /// Create an borrowed reference to a byte slice to the memory at `ptr` with a length of `len`.
    ///
    /// This will *not* be dropped or freed at the end of usaged.
    ///
    /// # Safety
    ///
    /// The `ptr` must be a valid offset in the WASM module for the slice of the specified length. It must abide by the same rules as a `&[u8]`.
    pub unsafe fn borrowed(ptr: &i32, len: i32) -> Borrowed<'_, Self> {
        Borrowed {
            val: Self(WasmSlice { ptr: *ptr, len }),
            ghost: PhantomData,
        }
    }

    /// Return the content as a slice of bytes
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Return the content as a str of utf8 or allocate and replace as necessary.
    ///
    /// See [`std::string::String::from_utf8_lossy`]
    pub fn from_utf8_lossy(&self) -> Cow<'_, str> {
        let string = self.as_bytes();
        String::from_utf8_lossy(string)
    }
}

impl WasmAllocated for WasmString {
    /// Return the WASM offset pointer
    fn ptr(&self) -> i32 {
        self.0.ptr
    }

    /// Return the WASM offset pointer
    fn len(&self) -> i32 {
        self.0.len
    }
}

impl From<String> for Owned<WasmString> {
    #[inline]
    fn from(s: String) -> Self {
        let bytes = s.into_bytes();
        Owned(WasmString(Owned::<WasmSlice>::from(bytes).0))
    }
}

/// An Owned item will be dropped
#[repr(transparent)]
pub struct Owned<T: WasmAllocated>(T);

impl<T: WasmAllocated> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe { __dealloc_bytes(self.0.ptr() as u32, self.0.len() as u32) };
    }
}

impl<T: WasmAllocated> Deref for Owned<T> {
    type Target = T;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A Borrowed item will not be dropped
#[repr(transparent)]
pub struct Borrowed<'a, T: WasmAllocated> {
    val: T,
    ghost: PhantomData<&'a T>,
}

impl<'a, T: WasmAllocated> Deref for Borrowed<'a, T> {
    type Target = T;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}
