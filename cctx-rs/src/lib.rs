#![feature(trivial_bounds)]
#![feature(custom_attribute)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate tokio;
extern crate tokio_core;
extern crate tokio_reactor;

extern crate failure;
extern crate futures;
#[macro_use]
extern crate try_future;

extern crate hyper;
extern crate hyper_tls;

pub mod prelude;

#[macro_use]
pub mod base;
pub mod bitfinex;