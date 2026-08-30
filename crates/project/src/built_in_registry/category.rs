use std::fmt::{Display, Formatter};

/// A category to a piece of data within the store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Category {
    /// The segments forming the category path.
    segments: &'static [&'static str],
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some((first, remaining)) = self.segments().split_first() {
            write!(f, "{first}")?;
            for segment in remaining {
                write!(f, "/{segment}")?;
            }
        }
        Ok(())
    }
}

impl Category {
    /// Creates a category from statically allocated segments.
    #[doc(hidden)]
    #[must_use]
    pub const fn __new(segments: &'static [&'static str]) -> Self {
        Self { segments }
    }

    /// Returns all segments forming the category path.
    #[must_use]
    pub const fn segments(&self) -> &'static [&'static str] {
        self.segments
    }
}

/// Creates a category from one to five static segments.
macro_rules! category {
    ($($segment:literal),+ $(,)?) => {
        const {
            let segments: &'static [&'static str] = &[$($segment),+];

            assert!(segments.len() <= 5, "a category can contain at most five segments");

            #[allow(clippy::disallowed_methods)]
            $crate::built_in_registry::Category::__new(segments)
        }
    };
}

pub(crate) use category;

impl PartialEq<&Category> for Category {
    fn eq(&self, other: &&Category) -> bool {
        self == *other
    }
}

impl PartialEq<Category> for &Category {
    fn eq(&self, other: &Category) -> bool {
        *self == other
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_categories() {
        // Object category
        let category = category!("obj");
        assert_eq!(category.to_string(), "obj");

        // parameter category
        let category = category!("obj", "prop");
        assert_eq!(category.to_string(), "obj/prop");

        // Map entry category
        let category = category!("obj", "prop", "key");
        assert_eq!(category.to_string(), "obj/prop/key");

        // Map entry item category from parameter
        let category = category!("obj", "prop", "item");
        assert_eq!(category.to_string(), "obj/prop/item");

        // Map entry item category from map entry
        let category = category!("obj", "prop", "key", "item");
        assert_eq!(category.to_string(), "obj/prop/key/item");
    }

    #[test]
    fn test_category_segments() {
        let p1 = category!("obj", "prop");
        assert_eq!(p1.to_string(), "obj/prop");

        let p2 = category!("obj", "prop", "key");
        assert_eq!(p2.to_string(), "obj/prop/key");

        let p3 = category!("obj", "prop", "key", "item");
        assert_eq!(p3.to_string(), "obj/prop/key/item");

        let p4 = category!("obj", "prop", "key", "item", "nested");
        assert_eq!(p4.to_string(), "obj/prop/key/item/nested");
    }

    #[test]
    fn test_category_equality() {
        let p1 = category!("obj", "prop", "key");
        let p2 = category!("obj", "prop", "key");
        let p3 = category!("obj", "prop", "other");
        let p4 = category!("other", "prop", "key");

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert_ne!(p1, p4);

        let p5 = category!("obj");
        let p6 = category!("obj");
        assert_eq!(p5, p6);

        // Reference comparisons
        assert_eq!(p1, &p2);
        assert_eq!(&p1, p2);
        assert_eq!(&p1, &p2);
    }
}
