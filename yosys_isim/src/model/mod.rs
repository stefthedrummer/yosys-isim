pub mod module;
pub use module::*;
pub mod port;
pub use port::*;
pub mod cell;
pub use cell::*;

pub type HCell = usize;
pub type HWire = usize;

// #[derive(Debug, Copy, Clone)]
// pub enum HWireOrConst {
//     HWire(HWire),
//     Const(Logic),
// }
