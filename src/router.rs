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

use slack;

pub struct Router;

#[allow(unused_variables)]
impl slack::EventHandler for Router {
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

