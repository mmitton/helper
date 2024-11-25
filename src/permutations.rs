use std::marker::PhantomData;

#[derive(Debug)]
struct Group<T>
where
    T: Copy + std::fmt::Debug,
{
    spans: Vec<Span>,
    v: T,
}

#[derive(Copy, Clone, Debug)]
enum Span {
    Some(u8),
    None(u8),
}

impl<T: Copy + std::fmt::Debug> Group<T> {
    fn new(v: T, len: usize, total_len: usize) -> Self {
        assert!(total_len >= len);
        let mut spans = Vec::with_capacity(len * 2);
        if total_len != len {
            spans.push(Span::None((total_len - len) as u8));
        }
        spans.push(Span::Some(len as u8));
        Self { spans, v }
    }

    // return true when resets
    fn next(&mut self) -> bool {
        fn next(spans: &mut Vec<Span>, idx: usize, extra_empty: u8) {
            match (spans[idx - 1], spans[idx]) {
                (Span::None(n), Span::Some(s)) => {
                    // Move 1 from Span::Some up and send all behind it
                    // to the end.
                    if n > 1 {
                        spans[idx - 1] = Span::None(n - 1);
                        spans[idx] = Span::Some(1);
                        spans.truncate(idx + 1);
                    } else if idx > 1 {
                        if let Span::Some(a) = spans[idx - 2] {
                            spans[idx - 2] = Span::Some(a + 1);
                            spans.truncate(idx - 1);
                        } else {
                            unreachable!();
                        }
                    } else {
                        spans.truncate(1);
                        spans[0] = Span::Some(1);
                    }
                    spans.push(Span::None(extra_empty + 1));
                    if s - 1 > 0 {
                        spans.push(Span::Some(s - 1));
                    }
                }
                (Span::Some(_), Span::None(n)) => next(spans, idx - 1, n + extra_empty),
                _ => unreachable!(),
            }
        }
        match self.spans.len() {
            1 => true,
            2 if matches!(self.spans[0], Span::Some(_)) => {
                // Reached the end.
                // Swap the Span::Some and Span::None
                self.spans.swap(0, 1);
                true
            }
            _ => {
                // Start at the tail and recurse to the head
                let idx = self.spans.len() - 1;
                next(&mut self.spans, idx, 0);
                false
            }
        }
    }
}

pub struct GroupedPermutations<T>
where
    T: Copy + std::fmt::Debug,
{
    groups: Vec<Group<T>>,
    next_vec: Vec<T>,
    next_filled_in: Vec<bool>,
    next_spans: Vec<(usize, Span, T)>,
    done: bool,
}

impl<T: Copy + std::fmt::Debug> GroupedPermutations<T> {
    pub fn new(groups: impl AsRef<[(T, usize)]>) -> Self {
        let groups_slice = groups.as_ref();
        let total_len = groups_slice.iter().map(|(_, len)| len).sum();
        let mut left = total_len;
        let next_vec = vec![groups_slice[0].0; total_len];

        let mut groups = Vec::new();
        for (v, len) in groups_slice.iter().copied() {
            groups.push(Group::new(v, len, left));
            left -= len;
        }
        groups.reverse();
        let v = groups_slice[0].0;

        Self {
            groups,
            next_vec,
            next_filled_in: vec![false; total_len],
            next_spans: vec![(0, Span::None(0), v); groups_slice.len()],
            done: false,
        }
    }

    fn tick(&mut self) {
        for group in &mut self.groups.iter_mut() {
            if !group.next() {
                return;
            }
        }
        self.done = true;
    }

    pub fn next_permutation(&mut self) -> Option<&[T]> {
        if self.done {
            return None;
        }

        for (group, next_span) in self.groups.iter().zip(self.next_spans.iter_mut()) {
            *next_span = (0, group.spans[0], group.v);
        }

        for next in self.next_vec.iter_mut() {
            for (idx, (span_idx, next_span, v)) in self.next_spans.iter_mut().enumerate().rev() {
                if matches!(next_span, Span::Some(0) | Span::None(0)) {
                    // fetch next
                    *span_idx += 1;
                    *next_span = self.groups[idx].spans[*span_idx];
                }
                match next_span {
                    Span::None(n) => *n -= 1,
                    Span::Some(n) => {
                        *n -= 1;
                        *next = *v;
                        break;
                    }
                }
            }
        }

        self.tick();
        Some(self.next_vec.as_slice())
    }
}

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

    #[test]
    fn grouped_perm_1() {
        #[derive(Copy, Clone, Debug, PartialEq)]
        enum Item {
            A,
            B,
        }

        let groups = [(Item::A, 3), (Item::B, 3)];

        fn get_string(slice: &[Item]) -> String {
            let mut s = String::with_capacity(slice.len());
            for v in slice.iter() {
                if matches!(v, Item::A) {
                    s.push('A');
                } else {
                    s.push('_');
                }
            }
            s
        }

        let expected = [
            "___AAA", "__A_AA", "__AA_A", "__AAA_", "_A__AA", "_A_A_A", "_A_AA_", "_AA__A",
            "_AA_A_", "_AAA__", "A___AA", "A__A_A", "A__AA_", "A_A__A", "A_A_A_", "A_AA__",
            "AA___A", "AA__A_", "AA_A__", "AAA___",
        ];

        let mut group_perm = super::GroupedPermutations::new(groups);
        let mut total = 0;
        println!();
        while let Some(next) = group_perm.next_permutation() {
            let got = get_string(next);
            let expected = expected.get(total).copied().unwrap_or("");
            total += 1;
            println!("Iter {total:02}:  Expected:{expected}  Got:{got}");
            assert_eq!(got, expected);
        }
        assert_eq!(total, 20);
    }

    #[test]
    fn grouped_perm_2() {
        #[derive(Copy, Clone, Debug, PartialEq)]
        enum Item {
            A,
            B,
            C,
        }

        let mut expected = std::collections::HashMap::new();
        expected.insert(1, 6);
        expected.insert(2, 90);
        expected.insert(3, 1680);
        expected.insert(4, 34650);
        expected.insert(5, 756756);

        for width in 1..=6 {
            let groups = [(Item::A, width), (Item::B, width), (Item::C, width)];

            let mut group_perm = super::GroupedPermutations::new(groups);
            let mut total = 0;
            while group_perm.next_permutation().is_some() {
                total += 1;
            }
            println!("{total} combinations for {groups:?}");
            if let Some(expected) = expected.get(&width) {
                assert_eq!(*expected, total);
            }
        }
    }
}
