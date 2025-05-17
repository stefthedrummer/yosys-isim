use crate::common::HasName;
use crate::common::Str8;
use crate::common::Vec4;
use std::marker::PhantomData;

pub trait Dir {}
#[derive(Debug, Clone, Copy)]
pub enum In {}
#[derive(Debug, Clone, Copy)]
pub enum Out {}
impl Dir for In {}
impl Dir for Out {}

#[derive(Debug, Clone)]
pub struct Port<D: Dir, W, const L: usize = 0> {
    pub name: Str8,
    pub wires: Vec4<W>,
    pub dir: PhantomData<D>,
}

impl<D: Dir, W> Port<D, W> {
    pub fn into_width<const L: usize>(self) -> Port<D, W, L> {
        Port {
            name: self.name,
            wires: self.wires,
            dir: self.dir,
        }
    }
}

impl<D: Dir, W> HasName for Port<D, W> {
    const LABEL: &'static str = "port";
    fn name(&self) -> &str {
        &self.name
    }
}
