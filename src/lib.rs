#![warn(missing_docs)]
#![doc(html_logo_url = "../../../uml/logo.png")]
//! # Legends Events guest registrations Record
//!
//! ## Description
//!
//! 
//!
//! ## Objectives
//!
//! The main goal of the **Legends registrations** is to:
//!   - Define a way to record the registrations data.
//!

/// Main registrations Module
pub mod contract;

/// custom error handler
pub mod error;

/// custom input output messages
pub mod msg;

/// state on the blockchain
pub mod state;