//! Generationa area is a vector-like data structure with validated hole reusage properties.
//! It's a light wrapper around vector, where each entry is versioned with a sequential index called generation.
//! It's useful for dynamic tree- and graph-like structures with dangling pointers.
//! The size of the arena is proportional to the max amount of concurrent entries.
//! The size of the arena cannot decrease, because it would risk access violation and invalidate pointers.

use std::iter::Enumerate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdxGen {
    idx: usize,
    gen: usize,
}

#[derive(Debug)]
struct OptEntry<T> {
    gen: usize,
    opt_value: Option<T>,
}

#[derive(Debug)]
pub struct RefEntry<'a, T> {
    pub idx_gen: IdxGen,
    pub value: &'a T,
}

#[derive(Debug)]
pub struct MutEntry<'a, T> {
    pub idx_gen: IdxGen,
    pub value: &'a mut T,
}

#[derive(Debug)]
pub struct Area<T> {
    data: Vec<OptEntry<T>>,
}

impl<T> Default for Area<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T> Area<T> {
    pub fn add(&mut self, value: T) -> IdxGen {
        let empty = self
            .data
            .iter()
            .enumerate()
            .find_map(|(idx, entry)| match entry.opt_value {
                Some(_) => None,
                None => Some(IdxGen {
                    idx,
                    gen: entry.gen,
                }),
            });

        match empty {
            Some(empty) => {
                let gen = empty.gen + 1;
                let idx = empty.idx;
                self.data[idx] = OptEntry {
                    gen,
                    opt_value: Some(value),
                };
                IdxGen { idx, gen }
            }
            None => {
                let gen = 0;
                let idx = self.data.len();
                self.data.push(OptEntry {
                    gen,
                    opt_value: Some(value),
                });
                IdxGen { idx, gen }
            }
        }
    }

    pub fn get(&self, idx_gen: IdxGen) -> Option<&T> {
        let opt_entry = self.data.get(idx_gen.idx)?;
        if opt_entry.gen != idx_gen.gen {
            return None;
        }
        let value = opt_entry.opt_value.as_ref()?;
        Some(value)
    }

    pub fn get_mut(&mut self, idx_gen: IdxGen) -> Option<&mut T> {
        let opt_entry = self.data.get_mut(idx_gen.idx)?;
        if opt_entry.gen != idx_gen.gen {
            return None;
        }
        let value = opt_entry.opt_value.as_mut()?;
        Some(value)
    }

    pub fn remove(&mut self, idx_gen: IdxGen) -> Option<T> {
        let opt_entry = self.data.get_mut(idx_gen.idx)?;
        if opt_entry.gen != idx_gen.gen {
            return None;
        }
        let value = opt_entry.opt_value.take()?;
        Some(value)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner: self.data.iter().enumerate(),
        }
    }
}

pub struct Iter<'a, T> {
    inner: Enumerate<std::slice::Iter<'a, OptEntry<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = RefEntry<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, entry)) = self.inner.next() {
            if let Some(value) = entry.opt_value.as_ref() {
                return Some(RefEntry {
                    idx_gen: IdxGen {
                        idx,
                        gen: entry.gen,
                    },
                    value: value,
                });
            } else {
                return None;
            }
        }
        None
    }
}

pub struct IterMut<'a, T> {
    inner: Enumerate<std::slice::IterMut<'a, OptEntry<T>>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = MutEntry<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((idx, entry)) = self.inner.next() {
            if let Some(value) = entry.opt_value.as_mut() {
                let idx_gen = IdxGen {
                    idx,
                    gen: entry.gen,
                };
                return Some(MutEntry {
                    idx_gen,
                    value: value,
                });
            } else {
                return None;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Area;
    use crate::genarea::IdxGen;

    #[test]
    fn test() {
        let mut area = Area::<String>::default();

        let a_ig = area.add("a".to_string());
        let a = area.remove(a_ig).unwrap();
        assert_eq!(a_ig, IdxGen { idx: 0, gen: 0 });
        assert_eq!(a, "a");

        let b_ig = area.add("b".to_string());
        let b = area.get(b_ig).unwrap();
        assert_eq!(b_ig, IdxGen { idx: 0, gen: 1 });
        assert_eq!(b, "b");

        let c_ig = area.add("c".to_string());
        let c = area.remove(c_ig).unwrap();
        assert_eq!(c_ig, IdxGen { idx: 1, gen: 0 });
        assert_eq!(c, "c");
    }
}
