//
// Copyright (c) Pirmin Kalberer. All rights reserved.
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
//

extern crate clap;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nickel;
extern crate open;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_json;

extern crate t_rex_core;
extern crate t_rex_service;

use t_rex_core::core;
use t_rex_core::datasource;
use t_rex_core::service;
use t_rex_core::cache;
use t_rex_service::mvt_service;
use t_rex_service::datasource_type;
use t_rex_service::read_qgs;

pub mod server;
