//--------------------------------------------------------------------------------------------------
// Chesster - a lichess 45+45 helper bot.
//
// Copyright (C) 2017 Lakin Wecker <lakin@wecker.ca>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//--------------------------------------------------------------------------------------------------

extern crate slack;
extern crate config;

pub mod router;

use router::{Router};

use std::env;

fn main() {
    // Create a new local configuration
    let mut c = config::Config::new();

    // Add 'Settings.toml'
    c.merge(config::File::new("Settings", config::FileFormat::Json).required(true)).expect("Settings.json MUST exist in the current directory");

    // Add 'Settings.$(RUST_ENV).toml`
    let name = format!("Settings-{}", env::var("env").unwrap_or("development".into()));
	println!("{:?}", name);
    c.merge(config::File::new(&name, config::FileFormat::Json).required(true)).unwrap();

    // Add environment variables that begin with APP_
    //c.merge(config::Environment::new("CHESSTER")).unwrap();

    let api_key = c.get_str("slack_tokens.lichess4545").expect("slack_tokens.lichess4545 must be set to a valid string");
	println!("{:?}", api_key);

    let mut router = Router;
    let mut cli = slack::RtmClient::new(&api_key);
    let r = cli.login_and_run::<Router>(&mut router);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}
