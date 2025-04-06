pub type MemoryReader = myinterface::myinterface::types::MemoryReader;
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_calculate_sha256_cabi<T: Guest>(arg0: i32) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let handle0;
    let result1 = T::calculate_sha256({
        handle0 = myinterface::myinterface::types::MemoryReader::from_handle(
            arg0 as u32,
        );
        &handle0
    });
    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let vec3 = (result1.into_bytes()).into_boxed_slice();
    let ptr3 = vec3.as_ptr().cast::<u8>();
    let len3 = vec3.len();
    ::core::mem::forget(vec3);
    *ptr2.add(4).cast::<usize>() = len3;
    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
    ptr2
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_calculate_sha256<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
}
pub trait Guest {
    fn calculate_sha256(r: &MemoryReader) -> _rt::String;
}
#[doc(hidden)]
macro_rules! __export_world_app_world_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "calculate-sha256"] unsafe extern "C" fn
        export_calculate_sha256(arg0 : i32,) -> * mut u8 { $($path_to_types)*::
        _export_calculate_sha256_cabi::<$ty > (arg0) } #[export_name =
        "cabi_post_calculate-sha256"] unsafe extern "C" fn
        _post_return_calculate_sha256(arg0 : * mut u8,) { $($path_to_types)*::
        __post_return_calculate_sha256::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_app_world_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 8]);
#[allow(dead_code)]
pub mod myinterface {
    #[allow(dead_code)]
    pub mod myinterface {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct MemoryReader {
                handle: _rt::Resource<MemoryReader>,
            }
            impl MemoryReader {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for MemoryReader {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "myinterface:myinterface/types")]
                        extern "C" {
                            #[link_name = "[resource-drop]memory-reader"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl MemoryReader {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read_byte(&self) -> Result<u8, ()> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "myinterface:myinterface/types")]
                        extern "C" {
                            #[link_name = "[method]memory-reader.read-byte"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    l2 as u8
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl MemoryReader {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read_max_n_bytes(&self, n: u32) -> Result<_rt::Vec<u8>, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "myinterface:myinterface/types")]
                        extern "C" {
                            #[link_name = "[method]memory-reader.read-max-n-bytes"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&n), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub use alloc_crate::vec::Vec;
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_app_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_app_world_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_app_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:app:app-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 406] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x96\x02\x01A\x02\x01\
A\x07\x01B\x09\x04\0\x0dmemory-reader\x03\x01\x01h\0\x01j\x01}\0\x01@\x01\x04sel\
f\x01\0\x02\x04\0\x1f[method]memory-reader.read-byte\x01\x03\x01p}\x01j\x01\x04\0\
\x01@\x02\x04self\x01\x01ny\0\x05\x04\0&[method]memory-reader.read-max-n-bytes\x01\
\x06\x03\x01\x1dmyinterface:myinterface/types\x05\0\x02\x03\0\0\x0dmemory-reader\
\x03\0\x0dmemory-reader\x03\0\x01\x01h\x02\x01@\x01\x01r\x03\0s\x04\0\x10calcula\
te-sha256\x01\x04\x04\x01\x17component:app/app-world\x04\0\x0b\x0f\x01\0\x09app-\
world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.\
0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
