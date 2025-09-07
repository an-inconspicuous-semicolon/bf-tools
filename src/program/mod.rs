pub mod basic;

pub trait Program<T> {
    fn get(&self, index: usize) -> Option<T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
