//! Raw, unsafe declarations mirroring `include/sleekfft/sleekfft.h`.

use std::{marker::PhantomData, marker::PhantomPinned, os::raw::c_int};

/// Opaque `sleekfft_context`. Only ever handled behind a pointer.
#[repr(C)]
pub struct Context {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

pub const SLEEKFFT_OK: c_int = 0;
pub const SLEEKFFT_ERR_BAD_INPUT: c_int = -2;

unsafe extern "C" {
    pub unsafe fn sleekfft_create_context() -> *mut Context;
    /// The C prototype takes `const sleekfft_context *`; constness does not affect the ABI.
    pub unsafe fn sleekfft_destroy_context(context: *mut Context) -> c_int;
    pub unsafe fn sleekfft_forward_transform(
        context: *mut Context,
        xr: *mut f64,
        xi: *mut f64,
        k: usize,
    ) -> c_int;
    pub unsafe fn sleekfft_inverse_transform(
        context: *mut Context,
        xr: *mut f64,
        xi: *mut f64,
        k: usize,
    ) -> c_int;
    pub unsafe fn sleekfft_cyclic_conv(
        context: *mut Context,
        x1: *mut f64,
        x2: *mut f64,
        k: usize,
    ) -> c_int;
}
