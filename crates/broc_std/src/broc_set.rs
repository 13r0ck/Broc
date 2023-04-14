use crate::broc_dict::BrocDict;
use core::{
    fmt::{self, Debug},
    hash::Hash,
};

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BrocSet<T>(BrocDict<T, ()>);

impl<T> BrocSet<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[allow(unused)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(BrocDict::with_capacity(capacity))
    }

    #[allow(unused)]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter_keys()
    }
}

impl<T: Hash> FromIterator<T> for BrocSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(into_iter: I) -> Self {
        Self(BrocDict::from_iter(
            into_iter.into_iter().map(|elem| (elem, ())),
        ))
    }
}

impl<T: Debug> Debug for BrocSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BrocSet ")?;

        f.debug_set().entries(self.iter()).finish()
    }
}
