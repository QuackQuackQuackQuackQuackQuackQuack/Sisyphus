

pub struct Limit<I>
where
    I : Iterator
{
    pub(super) iter      : I,
    pub(super) remaining : usize
}

impl<I> Iterator for Limit<I>
where
    I : Iterator
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(remaining) = self.remaining.checked_sub(1) {
            self.remaining = remaining;
            self.iter.next()
        } else { None }
    }
}
