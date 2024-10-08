//!
//! GURPS Character Generator (gurpschgen) data handler library.
//! 
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
extern crate glob;
pub mod attrib;
pub mod edition;
pub mod config;
pub mod modifier;
pub mod ch;
pub mod misc;
pub mod gender;
pub mod adq;
pub mod dta;
pub mod context;
pub mod equipment;
pub mod damage;
pub mod skill;
