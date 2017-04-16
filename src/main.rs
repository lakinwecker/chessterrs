//
// Copyright 2014-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//
// This is a simple example of using slack-rs.
// You can run it with `cargo run --example slack_example -- <api_key>`
//
// NOTE: This will post in the #general channel of the account you connect
// to.
//

extern crate slack;
extern crate config;

use std::env;

struct MyHandler;


#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &mut slack::RtmClient, event: Result<slack::Event, slack::Error>, raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);
    }

    fn on_ping(&mut self, cli: &mut slack::RtmClient) {
        println!("on_ping");
    }

    fn on_close(&mut self, cli: &mut slack::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &mut slack::RtmClient) {
        println!("on_connect");
        // Do a few things using the api:
        // send a message over the real time api websocket
        let _ = cli.send_message("#unstable_bot-lonewolf", "Hello world Rust! (rtm)");
        // post a message as a user to the web api
        let _ = cli.post_message("#unstable_bot-lonewolf", "hello world Rust! (postMessage)", None);
        // set a channel topic via the web api
        // let _ = cli.set_topic("#general", "bots rule!");
    }
}

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

    let mut handler = MyHandler;
    let mut cli = slack::RtmClient::new(&api_key);
    let r = cli.login_and_run::<MyHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}
