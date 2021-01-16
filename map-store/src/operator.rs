/// Trait representing certain operation to be applied on `MapStore`.
pub trait Operator<V> {
    /// Apply this operation on `value`.
    fn apply(&self, value: &mut V);
}

pub struct IncrementOp {}

impl Operator<u32> for IncrementOp {
    fn apply(&self, value: &mut u32) {
        *value += 1;
    }
}
