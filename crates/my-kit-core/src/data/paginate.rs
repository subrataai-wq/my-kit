//! Non-generic pagination helper and slice windowing.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pagination {
    pub page: usize,
    pub page_size: usize,
    pub total_items: usize,
}

impl Pagination {
    pub fn new(page: usize, page_size: usize, total_items: usize) -> Self {
        let page = page.max(1);
        let page_size = page_size.max(1);
        Self {
            page,
            page_size,
            total_items,
        }
    }

    pub fn total_pages(&self) -> usize {
        if self.total_items == 0 {
            1
        } else {
            self.total_items.div_ceil(self.page_size)
        }
    }

    pub fn has_next(&self) -> bool {
        self.page < self.total_pages()
    }

    pub fn has_prev(&self) -> bool {
        self.page > 1
    }

    pub fn offset(&self) -> usize {
        (self.page - 1) * self.page_size
    }

    /// Extract the slice of items for the current page without cloning elements.
    pub fn slice<'a, T>(&self, items: &'a [T]) -> &'a [T] {
        let start = self.offset().min(items.len());
        let end = (start + self.page_size).min(items.len());
        &items[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_calculations() {
        let p = Pagination::new(1, 10, 25);
        assert_eq!(p.total_pages(), 3);
        assert!(p.has_next());
        assert!(!p.has_prev());

        let p2 = Pagination::new(3, 10, 25);
        assert!(!p2.has_next());
        assert!(p2.has_prev());
    }

    #[test]
    fn slice_windowing() {
        let items: Vec<i32> = (1..=25).collect();
        let p1 = Pagination::new(1, 10, 25);
        assert_eq!(p1.slice(&items), &(1..=10).collect::<Vec<_>>()[..]);

        let p3 = Pagination::new(3, 10, 25);
        assert_eq!(p3.slice(&items), &[21, 22, 23, 24, 25]);

        let p_out_of_bounds = Pagination::new(5, 10, 25);
        assert_eq!(p_out_of_bounds.slice(&items), &[] as &[i32]);
    }
}
