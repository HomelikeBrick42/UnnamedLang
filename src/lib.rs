use std::sync::atomic::AtomicUsize;

pub mod binding;
pub mod syntax;

pub(crate) fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}
