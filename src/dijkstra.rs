use crate::{HashMap, HashSet};
use std::{collections::BTreeMap, hash::Hash};

use crate::Integer;

#[derive(Default)]
struct SeenHash<C, T>
where
    C: Integer,
    T: Hash + Eq + Copy,
{
    hash: HashMap<T, (C, Vec<T>)>,
}

impl<C, T> SeenHash<C, T>
where
    C: Integer,
    T: Hash + Eq + Copy,
{
    fn new(start: T) -> Self {
        let mut hash = HashMap::default();
        hash.insert(start, (C::ZERO, Vec::new()));
        Self { hash }
    }

    fn is_cheaper(&mut self, from: T, to: T, cost: C) -> bool {
        let best_cost = self.hash.entry(to).or_insert((C::MAX, Vec::new()));

        use std::cmp::Ordering;
        match cost.cmp(&best_cost.0) {
            Ordering::Greater => false,
            Ordering::Equal => {
                best_cost.1.push(from);
                false
            }
            Ordering::Less => {
                best_cost.0 = cost;
                best_cost.1.clear();
                best_cost.1.push(from);
                true
            }
        }
    }

    fn has_cheaper(&self, cost: C, v: &T) -> bool {
        if let Some(seen_cost) = self.hash.get(v) {
            if seen_cost.0 < cost {
                return true;
            }
        }

        false
    }

    fn get_paths_from(&self, target: T) -> Vec<Vec<T>> {
        let mut paths = Vec::new();

        let mut work: Vec<(T, Vec<T>)> = vec![(target, vec![target])];
        while let Some((cur, mut path)) = work.pop() {
            if let Some(seen_entry) = self.hash.get(&cur) {
                if seen_entry.1.is_empty() {
                    path.reverse();
                    paths.push(path);
                } else {
                    for from in seen_entry.1.iter() {
                        let mut path = path.clone();
                        path.push(*from);
                        work.push((*from, path));
                    }
                }
            }
        }

        paths
    }
}

pub struct Dijkstra {}

impl Dijkstra {
    fn dijkstra_internal<C, T, N, NI, R>(start: T, next: N, result: R)
    where
        C: Integer,
        T: Hash + Eq + Copy,
        N: FnMut(T) -> NI,
        NI: Iterator<Item = (C, T, bool)>,
        R: FnMut(C, T, &SeenHash<C, T>) -> bool,
    {
        let mut work: BTreeMap<C, HashSet<(T, bool)>> = BTreeMap::new();
        let mut next = next;
        let mut result = result;
        work.entry(C::ZERO).or_default().insert((start, false));

        let mut seen: SeenHash<C, T> = SeenHash::new(start);

        let mut found_result = false;
        while let Some((cost, mut cost_work)) = work.pop_first() {
            for (cur, is_target) in cost_work.drain() {
                if is_target && result(cost, cur, &seen) {
                    found_result = true;
                }
                if seen.has_cheaper(cost, &cur) {
                    continue;
                }
                for (next_cost, next_t, is_target) in next(cur) {
                    let next_cost = cost + next_cost;
                    if seen.is_cheaper(cur, next_t, next_cost) {
                        work.entry(next_cost)
                            .or_default()
                            .insert((next_t, is_target));
                    }
                }
            }
            if found_result {
                break;
            }
        }
    }

    pub fn find_first<C, T, N, NI>(start: T, next: N) -> Option<(C, T)>
    where
        C: Integer,
        T: Hash + Eq + Copy,
        N: FnMut(T) -> NI,
        NI: Iterator<Item = (C, T, bool)>,
    {
        let mut result: Option<(C, T)> = None;
        Self::dijkstra_internal(start, next, |cost, target, _| {
            result = Some((cost, target));
            true
        });
        result
    }

    pub fn find_first_paths<C, T, N, NI>(start: T, next: N) -> Option<(C, Vec<Vec<T>>)>
    where
        C: Integer,
        T: Hash + Eq + Copy,
        N: FnMut(T) -> NI,
        NI: Iterator<Item = (C, T, bool)>,
    {
        let mut result: Option<(C, Vec<Vec<T>>)> = None;
        Self::dijkstra_internal(start, next, |cost, target, seen| {
            let (_, paths) = result.get_or_insert_with(|| (cost, Vec::new()));

            for path in seen.get_paths_from(target) {
                paths.push(path);
            }

            true
        });

        result
    }

    pub fn find_all<C, T, N, NI>(start: T, next: N) -> Vec<(C, T)>
    where
        C: Integer,
        T: Hash + Eq + Copy,
        N: FnMut(T) -> NI,
        NI: Iterator<Item = (C, T, bool)>,
    {
        let mut result = Vec::new();
        Self::dijkstra_internal(start, next, |cost, target, _| {
            result.push((cost, target));
            false
        });
        result
    }

    pub fn find_all_paths<C, T, N, NI>(start: T, next: N) -> Vec<(C, Vec<Vec<T>>)>
    where
        C: Integer,
        T: Hash + Eq + Copy,
        N: FnMut(T) -> NI,
        NI: Iterator<Item = (C, T, bool)>,
    {
        let mut result = Vec::new();
        Self::dijkstra_internal(start, next, |cost, target, seen| {
            result.push((cost, seen.get_paths_from(target)));
            false
        });

        result
    }
}
