use crate::{HashMap, HashSet};
use std::{collections::BTreeMap, hash::Hash};

use crate::Integer;

pub struct Dijkstra {}

impl Dijkstra {
    fn dijkstra_internal<C, T, N, NI, R>(start: T, next: N, result: R)
    where
        C: Integer,
        T: Hash + Eq + Clone,
        N: FnMut(T) -> NI,
        NI: Iterator<Item = (C, T, bool)>,
        R: FnMut(C, T) -> bool,
    {
        let mut work: BTreeMap<C, HashSet<(T, bool)>> = BTreeMap::new();
        let mut next = next;
        let mut result = result;
        work.entry(C::ZERO)
            .or_default()
            .insert((start.clone(), false));

        let mut seen: HashMap<T, C> = HashMap::default();
        seen.insert(start, C::ZERO);

        while let Some((cost, mut cost_work)) = work.pop_first() {
            for (cur, is_target) in cost_work.drain() {
                if is_target && result(cost, cur.clone()) {
                    return;
                }
                if let Some(seen_cost) = seen.get(&cur) {
                    if seen_cost < &cost {
                        continue;
                    }
                }
                for (next_cost, next_t, is_target) in next(cur) {
                    let cost = cost + next_cost;
                    let seen_cost = seen.entry(next_t.clone()).or_insert(C::MAX);
                    if *seen_cost <= cost {
                        continue;
                    }
                    *seen_cost = cost;
                    work.entry(cost).or_default().insert((next_t, is_target));
                }
            }
        }
    }

    pub fn find_first<C, T, N, NI>(start: T, next: N) -> Option<(C, T)>
    where
        C: Integer,
        T: Hash + Eq + Clone,
        N: FnMut(T) -> NI,
        NI: Iterator<Item = (C, T, bool)>,
    {
        let mut result: Option<(C, T)> = None;
        Self::dijkstra_internal(start, next, |cost, target| {
            result = Some((cost, target));
            true
        });
        result
    }

    pub fn find_all<C, T, N, NI>(start: T, next: N) -> Vec<(C, T)>
    where
        C: Integer,
        T: Hash + Eq + Clone,
        N: FnMut(T) -> NI,
        NI: Iterator<Item = (C, T, bool)>,
    {
        let mut result = Vec::new();
        Self::dijkstra_internal(start, next, |cost, target| {
            result.push((cost, target));
            false
        });
        result
    }
}
