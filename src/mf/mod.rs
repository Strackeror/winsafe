#![cfg_attr(docsrs, doc(cfg(feature = "mf")))]

pub(in crate::mf) mod ffi;
pub(crate) mod privs;
pub mod co;

mod com_interfaces;
mod funcs;
mod structs;

pub mod decl {
	pub use super::com_interfaces::decl::*;
	pub use super::funcs::*;
	pub use super::structs::*;
}

pub mod traits {
	pub use super::com_interfaces::traits::*;
}

pub mod vt {
	pub use super::com_interfaces::vt::*;
}