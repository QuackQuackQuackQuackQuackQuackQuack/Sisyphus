mod limit;
use limit::*;


pub trait IteratorExt : Iterator + Sized {

    fn limit(self, count : usize) -> Limit<Self>;

    fn next_n_exact(&mut self, count : usize) -> Option<Vec<Self::Item>>;

}

impl<I> IteratorExt for I
where
    I : Iterator
{

    fn limit(self, count : usize) -> Limit<Self> {
        Limit { iter : self, remaining : count }
    }

    fn next_n_exact(&mut self, count : usize) -> Option<Vec<Self::Item>> {
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            out.push(self.next()?);
        }
        Some(out)
    }

}
