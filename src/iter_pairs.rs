pub trait IterPairs<I: Iterator>
where
    I::Item: Copy,
{
    fn pairs(self) -> IterPair<I>;
}

impl<I> IterPairs<I> for I
where
    I: Iterator,
    I::Item: Copy,
{
    fn pairs(mut self) -> IterPair<I> {
        let last = self.next();
        IterPair { iter: self, last }
    }
}

pub struct IterPair<I>
where
    I: Iterator,
    I::Item: Copy,
{
    iter: I,
    last: Option<I::Item>,
}

impl<I: Iterator> Iterator for IterPair<I>
where
    I::Item: Copy,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.last, self.iter.next()) {
            (Some(last), Some(next)) => {
                self.last = Some(next);
                Some((last, next))
            }
            _ => None,
        }
    }
}
