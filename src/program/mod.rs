pub mod basic;
pub mod semicolon_compressed;

pub trait Program<T> {
    fn get(&self, index: usize) -> Option<T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
