use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Span {
    start: usize,
    size: usize,
}

impl Span {
    pub(crate) fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            size: size.max(1),
        }
    }

    pub(crate) fn start(&self) -> usize {
        self.start
    }

    pub(crate) fn end(&self) -> usize {
        self.start + self.size
    }

    pub(crate) fn join(&self, other: &Span) -> Span {
        let new_start = self.start.min(other.start);
        let new_end = self.end().max(other.end());
        let new_size = (new_end - new_start).max(1);
        Span::new(new_start, new_size)
    }

    #[allow(dead_code)]
    pub(crate) fn overlaps(&self, other: &Span) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.start, self.end())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SpanSet {
    indices: Vec<Span>,
}

impl SpanSet {
    pub(crate) fn new() -> Self {
        Self {
            indices: Vec::new(),
        }
    }

    pub(crate) fn from_span(index: Span) -> Self {
        Self {
            indices: vec![index],
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_with_indices(indices: Vec<Span>) -> Self {
        let mut index_set = Self { indices };
        index_set.sort_and_merge();
        index_set
    }

    fn sort_and_merge(&mut self) {
        self.indices.sort_by_key(|index| index.start());
        let mut merged_indices: Vec<Span> = Vec::new();
        for index in &self.indices {
            if let Some(last) = merged_indices.last_mut() {
                if index.overlaps(last) {
                    *last = last.join(index);
                } else {
                    merged_indices.push(*index);
                }
            } else {
                merged_indices.push(*index);
            }
        }
        self.indices = merged_indices;
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Span> {
        self.indices.iter()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    #[allow(dead_code)]
    pub(crate) fn add(&mut self, index: Span) {
        self.indices.push(index);
        self.sort_and_merge();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join() {
        let index1 = Span::new(0, 5);
        let index2 = Span::new(3, 4);
        let joined_index = index1.join(&index2);
        assert_eq!(joined_index.start(), 0);
        assert_eq!(joined_index.end(), 7);
    }

    #[test]
    fn test_join_overlapping() {
        let index1 = Span::new(0, 5);
        let index2 = Span::new(3, 4);
        let joined_index = index1.join(&index2);
        assert_eq!(joined_index.start(), 0);
        assert_eq!(joined_index.end(), 7);
    }

    #[test]
    fn test_join_non_overlapping() {
        let index1 = Span::new(0, 5);
        let index2 = Span::new(6, 4);
        let joined_index = index1.join(&index2);
        assert_eq!(joined_index.start(), 0);
        assert_eq!(joined_index.end(), 10);
    }

    #[test]
    fn test_sort_and_merge() {
        let mut index_set = SpanSet::new();
        index_set.add(Span::new(0, 5));
        index_set.add(Span::new(3, 4));
        index_set.add(Span::new(10, 2));
        index_set.add(Span::new(9, 3));
        index_set.sort_and_merge();
        assert_eq!(index_set.indices.len(), 2);
        assert_eq!(index_set.indices[0].start(), 0);
        assert_eq!(index_set.indices[0].end(), 7);
        assert_eq!(index_set.indices[1].start(), 9);
        assert_eq!(index_set.indices[1].end(), 12);
    }

    #[test]
    fn test_from_index() {
        let index = Span::new(0, 5);
        let index_set = SpanSet::from_span(index);
        assert_eq!(index_set.indices.len(), 1);
        assert_eq!(index_set.indices[0].start(), 0);
        assert_eq!(index_set.indices[0].end(), 5);
    }

    #[test]
    fn test_new_with_indices() {
        let indices = vec![Span::new(0, 5), Span::new(3, 4), Span::new(10, 2)];
        let index_set = SpanSet::new_with_indices(indices);
        assert_eq!(index_set.indices.len(), 2);
        assert_eq!(index_set.indices[0].start(), 0);
        assert_eq!(index_set.indices[0].end(), 7);
        assert_eq!(index_set.indices[1].start(), 10);
        assert_eq!(index_set.indices[1].end(), 12);
    }
}
