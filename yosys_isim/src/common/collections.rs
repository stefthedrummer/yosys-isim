use more_collections::SmallSet;
use small_str::SmallStr;
use smallvec::SmallVec;

pub type Vec4<T> = SmallVec<[T; 4]>;
pub type Vec2<T> = SmallVec<[T; 2]>;
pub type Vec32<T> = SmallVec<[T; 32]>;
pub type Set4<T> = SmallSet<T, 4>;

pub type Str8 = SmallStr<8>;
