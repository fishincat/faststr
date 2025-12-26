use core::slice;
use std::{ops::Deref, sync::Arc};

use bytes::{Buf, Bytes};

// Vtable must enforce this behavior
unsafe impl Send for BytesRef {}
unsafe impl Sync for BytesRef {}

#[derive(Clone)]
pub struct BytesRef {
    pub(crate) ptr: *const u8,
    pub(crate) len: usize,
    pub(crate) data: Arc<Bytes>,
}

impl From<Bytes> for BytesRef {
    #[inline]
    fn from(data: Bytes) -> Self {
        BytesRef {
            ptr: data.as_ptr(),
            len: data.len(),
            data: Arc::new(data),
        }
    }
}

impl From<BytesRef> for Bytes {
    #[inline]
    fn from(value: BytesRef) -> Self {
        let original_start = value.data.as_ptr();
        let offset = unsafe { value.ptr.offset_from(original_start) } as usize;
        value.data.slice(offset..offset + value.len)
    }
}

impl BytesRef {
    pub unsafe fn slice_ref(&self, subset: &[u8]) -> Self {
        Self {
            ptr: subset.as_ptr(),
            len: subset.len(),
            data: self.data.clone(),
        }
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    #[inline]
    unsafe fn inc_start(&mut self, by: usize) {
        // should already be asserted, but debug assert for tests
        debug_assert!(self.len >= by, "internal: inc_start out of bounds");
        self.len -= by;
        self.ptr = self.ptr.add(by);
    }

    pub fn split_to(&mut self, at: usize) -> Bytes {
        if at == 0 {
            // TODO: provenance?
            return Bytes::new();
        }

        assert!(
            at <= self.len(),
            "split_to out of bounds: {:?} <= {:?}",
            at,
            self.len(),
        );

        let original_start = self.data.as_ptr();
        let offset = unsafe { self.ptr.offset_from(original_start) } as usize;
        let ret = self.data.slice(offset..offset + at);

        unsafe { self.inc_start(at) };

        ret
    }
}

impl Deref for BytesRef {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Buf for BytesRef {
    #[inline]
    fn remaining(&self) -> usize {
        self.len
    }

    #[inline]
    fn chunk(&self) -> &[u8] {
        self.as_slice()
    }

    #[inline]
    fn advance(&mut self, cnt: usize) {
        assert!(
            cnt <= self.len,
            "cannot advance past `remaining`: {:?} <= {:?}",
            cnt,
            self.len,
        );

        unsafe {
            self.inc_start(cnt);
        }
    }
}
