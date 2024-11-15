use std::collections::BTreeMap;

#[derive(Default)]
pub struct PriorityVec<K, T, const SIZE: usize = 16> {
    map: BTreeMap<K, Vec<T>>,
    empty_vecs: Vec<Vec<T>>,
}

impl<K, T, const SIZE: usize> PriorityVec<K, T, SIZE>
where
    K: Ord + Copy + Clone,
{
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            empty_vecs: Vec::new(),
        }
    }

    pub fn insert(&mut self, k: K, t: T) {
        self.map
            .entry(k)
            .or_insert_with(|| {
                if let Some(v) = self.empty_vecs.pop() {
                    v
                } else {
                    Vec::with_capacity(SIZE)
                }
            })
            .push(t);
    }

    pub fn pop(&mut self) -> Option<(K, T)> {
        if let Some(mut entry) = self.map.first_entry() {
            let k = *entry.key();
            let vec = entry.get_mut();
            if let Some(t) = vec.pop() {
                if vec.is_empty() {
                    let empty_vec = self.map.pop_first().unwrap();
                    self.empty_vecs.push(empty_vec.1);
                }
                Some((k, t))
            } else {
                unreachable!(
                    "Should not be able to have an entry in the PriorityVec with zero items"
                );
            }
        } else {
            None
        }
    }
}
