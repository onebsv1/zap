use super::{Task};
use std::{
    any::Any,
    future::Future,
    sync::atomic::{AtomicUsize},
};

type FutureError = Box<dyn Any + Send + 'static>;

#[repr(C, usize)]
enum FutureData<F: Future> {
    Pending(F),
    Ready(F::Output),
    Error(FutureError),
}

#[repr(C)]
pub struct FutureTask<F: Future> {
    task: Task,
    ref_count: AtomicUsize,
    data: FutureData<F>,
}