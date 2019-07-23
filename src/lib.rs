#![forbid(unsafe_code)]

pub mod error;
pub mod observable;
pub mod operators;
pub mod subject;
pub mod observer;
pub mod subscription;

#[cfg(test)]
mod tests;