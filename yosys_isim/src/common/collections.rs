use more_collections::SmallSet;
use smallvec::SmallVec;

pub type Vec4<T> = SmallVec<[T; 4]>;
pub type Vec32<T> = SmallVec<[T; 32]>;
pub type Set4<T> = SmallSet<T, 4>;
