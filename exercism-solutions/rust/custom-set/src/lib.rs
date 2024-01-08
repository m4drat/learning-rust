use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    iter::FromIterator,
};

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T>
where
    T: Hash + Clone + PartialEq,
{
    data: Vec<Vec<T>>,
    total_elements: usize,
    cardinality: usize,
}

impl<T> CustomSet<T>
where
    T: Hash + Clone + PartialEq,
{
    const GROWTH_FACTOR: usize = 2;
    const BASE_CARDINALITY: usize = 16;
    const THRESHOLD_LOAD: f32 = 0.87;

    fn get_insertion_index(&self, elem: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        elem.hash(&mut hasher);
        hasher.finish() as usize % self.cardinality
    }

    fn get_list_for_element(&self, elem: &T) -> &Vec<T> {
        let bucket = self.get_insertion_index(elem);
        &self.data[bucket]
    }

    fn get_list_for_element_mut(&mut self, elem: &T) -> &mut Vec<T> {
        let bucket = self.get_insertion_index(elem);
        &mut self.data[bucket]
    }

    fn load_factor(&self) -> f32 {
        self.total_elements as f32 / self.cardinality as f32
    }

    fn rehash_if_needed(&mut self) -> bool {
        if self.load_factor() > Self::THRESHOLD_LOAD {
            let old_vec = self.data.clone();
            self.cardinality *= Self::GROWTH_FACTOR;
            self.data = vec![vec![]; self.cardinality];
            self.total_elements = 0;

            old_vec
                .iter()
                .flatten()
                .for_each(|elem| self.add(elem.clone()));

            return true;
        }

        false
    }

    pub fn new(_input: &[T]) -> Self {
        let mut set = Self {
            data: vec![vec![]; Self::BASE_CARDINALITY],
            total_elements: 0,
            cardinality: Self::BASE_CARDINALITY,
        };

        _input.iter().for_each(|elem| set.add(elem.clone()));

        set
    }

    pub fn contains(&self, element: &T) -> bool {
        let list: &Vec<T> = self.get_list_for_element(element);
        list.iter().any(|entry| entry == element)
    }

    pub fn add(&mut self, element: T) {
        if self.contains(&element) {
            return;
        }

        self.rehash_if_needed();
        let list = self.get_list_for_element_mut(&element);

        if list.iter().all(|entry| entry != &element) {
            list.push(element);
            self.total_elements += 1;
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().flatten().all(|elem| other.contains(elem))
    }

    pub fn is_empty(&self) -> bool {
        self.total_elements == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.data.iter().flatten().any(|elem| other.contains(elem))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        self.data
            .iter()
            .flatten()
            .filter_map(|elem| other.contains(elem).then_some(elem.clone()))
            .collect::<CustomSet<T>>()
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        self.data
            .iter()
            .flatten()
            .filter_map(|elem| (!other.contains(elem)).then_some(elem.clone()))
            .collect::<CustomSet<T>>()
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        other
            .data
            .iter()
            .flatten()
            .chain(self.data.iter().flatten())
            .cloned()
            .collect::<CustomSet<T>>()
    }
}

impl<T> FromIterator<T> for CustomSet<T>
where
    T: Hash + Clone + PartialEq,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = CustomSet::new(&[]);

        for item in iter {
            set.add(item);
        }

        set
    }
}
