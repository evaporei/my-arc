use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{self, AtomicUsize, Ordering};

pub struct MyArc<T> {
    ptr: NonNull<ArcInner<T>>,
    phantom: PhantomData<ArcInner<T>>,
}

struct ArcInner<T> {
    rc: AtomicUsize,
    data: T,
}

impl<T> MyArc<T> {
    pub fn new(data: T) -> Self {
        let boxed_inner = Box::new(ArcInner {
            rc: AtomicUsize::new(1),
            data,
        });

        let heap_ptr = Box::into_raw(boxed_inner);

        Self {
            ptr: NonNull::new(heap_ptr).unwrap(),
            phantom: PhantomData,
        }
    }
}

impl<T> Deref for MyArc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        let old_rc = inner.rc.fetch_add(1, Ordering::Relaxed);

        if old_rc >= isize::MAX as usize {
            std::process::abort();
        }

        Self {
            ptr: self.ptr,
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        if inner.rc.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        atomic::fence(Ordering::Acquire);

        unsafe { Box::from_raw(self.ptr.as_ptr()) };
    }
}

unsafe impl<T: Sync + Send> Send for MyArc<T> {}
unsafe impl<T: Sync + Send> Sync for MyArc<T> {}
