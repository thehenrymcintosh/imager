// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(unused)]
pub mod tool;
pub mod utils;
pub mod opt;
pub mod cli;
pub mod data;
pub mod server;
pub mod api;

use structopt::StructOpt;
use cli::Command;

fn main() {
    let cmd = Command::from_args();
    cmd.run();
}