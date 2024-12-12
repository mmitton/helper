pub trait IterPairs<I, Item, P: Iterator>
where
    Item: Copy,
    I: Iterator<Item = Item>,
    P: Iterator<Item = (Item, Item)>,
{
    fn pairs(self) -> IterPair<I>;
}

impl<I, Item, S> IterPairs<I, Item, IterPair<I>> for S
where
    Item: Copy,
    S: IntoIterator<Item = Item, IntoIter = I>,
    I: Iterator<Item = Item>,
{
    fn pairs(self) -> IterPair<I> {
        let mut iter = self.into_iter();
        let last = iter.next();
        IterPair { iter, last }
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

#[cfg(test)]
mod test {
    use super::IterPairs;

    #[test]
    fn pairs_iter() {
        let v = [1, 2, 3];
        let mut iter = v.iter().pairs();
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn pairs_slice() {
        let v = &[1, 2, 3];
        let mut iter = v.pairs();
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.next(), None);
    }
}
