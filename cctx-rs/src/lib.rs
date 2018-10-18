#![feature(trivial_bounds)]
#![feature(custom_attribute)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate tokio;
extern crate tokio_core;
extern crate tokio_reactor;

extern crate failure;
extern crate futures;

extern crate hyper;
extern crate hyper_tls;

use hyper::rt;

use tokio::prelude::*;

pub mod prelude;

pub mod base;
pub mod bitfinex;