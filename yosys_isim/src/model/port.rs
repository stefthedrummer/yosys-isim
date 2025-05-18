use crate::common::HasName;
use crate::common::Str8;
use crate::common::Vec4;
use std::marker::PhantomData;
use std::mem::transmute;

pub trait Dir {}
#[derive(Debug, Clone, Copy)]
pub enum In {}
#[derive(Debug, Clone, Copy)]
pub enum Out {}
impl Dir for In {}
impl Dir for Out {}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Port<D: Dir, W, const L: usize = 0> {
    pub name: Str8,
    pub wires: Vec4<W>,
    pub dir: PhantomData<D>,
}

impl<D: Dir, W, const L: usize> Port<D, W, L> {
    pub fn into_width<const _L: usize>(self) -> Port<D, W, _L> {
        Port {
            name: self.name,
            wires: self.wires,
            dir: self.dir,
        }
    }

    pub fn as_ref_0(&self) -> &Port<D, W, 0> {
        unsafe { transmute(self) }
    }
}

impl<D: Dir, W> HasName for Port<D, W> {
    const LABEL: &'static str = "port";
    fn name(&self) -> &str {
        &self.name
    }
}
