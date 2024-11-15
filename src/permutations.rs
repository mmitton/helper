use std::marker::PhantomData;

pub struct Permutations<T> {
    _phanton: PhantomData<T>,
}

impl<T> Permutations<T>
where
    T: Copy,
{
    fn perm<F>(array: &mut [T], size: usize, f: &mut F)
    where
        F: FnMut(&mut [T]),
    {
        // if size becomes 1 then prints the obtained
        // permutation
        if size == 1 {
            f(array);
            return;
        }

        for i in 0..size {
            Self::perm(array, size - 1, f);

            if size % 2 == 1 {
                // if size is odd, swap 0th i.e (first) and
                // (size-1)th i.e (last) element
                (array[0], array[size - 1]) = (array[size - 1], array[0]);
            } else {
                // If size is even, swap ith and
                // (size-1)th i.e (last) element
                (array[i], array[size - 1]) = (array[size - 1], array[i]);
            }
        }
    }

    pub fn iter<F>(array: &mut [T], mut f: F)
    where
        F: FnMut(&mut [T]),
    {
        Self::perm(array, array.len(), &mut f);
    }

    pub fn iter_skip_last<F>(array: &mut [T], mut f: F)
    where
        F: FnMut(&mut [T]),
    {
        Self::perm(array, array.len() - 1, &mut f);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn iter() {
        let mut array = vec![0, 1, 2, 3];
        let mut cnt = 0;
        super::Permutations::iter(&mut array, |a| {
            println!("{a:?}");
            cnt += 1
        });
        assert_eq!(cnt, 24)
    }

    #[test]
    fn iter_skip_last() {
        let mut array = vec![0, 1, 2, 3];
        let mut cnt = 0;
        super::Permutations::iter_skip_last(&mut array, |a| {
            println!("{a:?}");
            cnt += 1
        });
        assert_eq!(cnt, 6);
    }
}
