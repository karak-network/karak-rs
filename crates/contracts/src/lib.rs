pub mod core;
pub mod erc20;
pub mod registry;
pub mod stake_viewer;
pub mod vault;

// TODO: This only exists to keep backwards compatibility
pub use core::contract::Core;
pub use core::interface::ICore;
