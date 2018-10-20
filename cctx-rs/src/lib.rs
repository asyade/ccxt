#![feature(trivial_bounds)]
#![feature(custom_attribute)]
#![feature(associated_type_defaults)]

extern crate serde_derive;
extern crate serde_json;
extern crate chrono;

extern crate tokio;
extern crate tokio_core;
extern crate tokio_reactor;

pub extern crate failure;
pub extern crate futures;
#[macro_use]
extern crate try_future;

pub extern crate hyper;
extern crate hyper_tls;

pub mod prelude;

#[macro_use]
pub mod base;
//pub mod bitfinex;
pub mod bitmex;