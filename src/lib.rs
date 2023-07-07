use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;

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

unsafe impl<T: Sync + Send> Send for MyArc<T> {}
unsafe impl<T: Sync + Send> Sync for MyArc<T> {}
