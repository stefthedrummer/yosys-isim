use crate::common::SimError;
use std::ops::Deref;

pub trait HasName {
    fn name(&self) -> &str;
}

pub trait FindByName {
    type Item;
    fn find_by_name(self, name: &str) -> Result<Self::Item, SimError>;
}

impl<T: HasName, Item: Deref<Target = T>, Iter: Iterator<Item = Item>> FindByName for Iter {
    type Item = Item;

    fn find_by_name(mut self, name: &str) -> Result<Self::Item, SimError> {
        self.find(|it| it.name().eq(name))
            .ok_or_else(|| SimError::SimError {
                msg: format!("could not find [{}]", name),
            })
    }
}
