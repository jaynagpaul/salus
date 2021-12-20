#![warn(missing_docs)]
//! # Salus
//!
//! Salus is a web framework for Rust designed around being productive and safe.

mod argument;
mod handler;
mod json;
mod log;
mod request;
mod responder;
mod response;
mod route;
mod salus;
mod state;

pub use crate::salus::*;
pub use handler::Handler;
pub use json::Json;
pub use request::Request;
pub use responder::Responder;
pub use response::Response;
pub use route::Route;
pub use state::State;

/// An attribute macro for defining an async trait
pub use async_trait::async_trait;
/// HTTP Types and Traits
pub use http;
