// Copyright (c) 2019 Tim Perkins

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use slack::{Event, RtmClient};

struct QHandler;

impl slack::EventHandler for QHandler {
    fn on_connect(&mut self, _client: &RtmClient) {
        println!("on_connect");
    }

    fn on_close(&mut self, _client: &RtmClient) {
        println!("on_close");
    }

    fn on_event(&mut self, _client: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);
    }
}

pub struct QNicknameBot {
    api_token: String,
    handler: QHandler,
}

impl QNicknameBot {
    pub fn new(api_token: &str) -> QNicknameBot {
        let bot_id = QNicknameBot::get_bot_id(&api_token).unwrap();
        println!("BOT_ID = {:?}", bot_id);
        QNicknameBot {
            api_token: api_token.to_string(),
            handler: QHandler,
        }
    }

    fn get_bot_id(api_token: &str) -> Option<String> {
        let client = slack::api::requests::default_client().unwrap();
        let response = slack::api::users::list(&client, api_token, &Default::default());
        let users = response.as_ref().unwrap().members.as_ref().unwrap();
        let qn_bot_users: Vec<&slack::User> = users
            .iter()
            .filter(|user| user.is_bot.unwrap() && user.name.as_ref().unwrap() == "qnicknamebot")
            .collect();
        match qn_bot_users.as_slice() {
            [qn_bot_user] => Some(qn_bot_user.id.as_ref().unwrap().to_string()),
            _ => None,
        }
    }

    pub fn run(&mut self) -> Result<(), slack::Error> {
        RtmClient::login_and_run(&self.api_token, &mut self.handler)
    }
}
