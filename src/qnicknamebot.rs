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

use lazy_static::lazy_static;
use regex::Regex;
use slack::{Event, Event::Message, Message::Standard, RtmClient, Sender};
use std::borrow::Cow;
use std::io::{self, Write};

use crate::qnicknames::get_qnickname;

fn is_dm_channel(channel: &str) -> bool {
    match channel.chars().nth(0) {
        Some(first_char) => first_char == 'D',
        None => false,
    }
}

fn message_contains_mention(bot_id: &str, message_text: &str) -> bool {
    // Something like: "<@UFYA397T8> Translate: Tim"
    let bot_id_pattern = format!("<@{}>", bot_id);
    message_text.contains(&bot_id_pattern)
}

lazy_static! {
    static ref TRANSLATE_NAME_REGEX: Regex =
        Regex::new(r"(?m)[Tt]ranslate:[[:space:]]+([[:alpha:]].*)$").unwrap();
}

struct QHandler {
    bot_id: String,
    sender: Sender,
}

impl slack::EventHandler for QHandler {
    fn on_connect(&mut self, _client: &RtmClient) {
        println!("QNicknameBot connected!");
    }

    fn on_close(&mut self, _client: &RtmClient) {
        // Do nothing
    }

    fn on_event(&mut self, _client: &RtmClient, event: Event) {
        let general_message = match &event {
            Message(m) => m.as_ref(),
            _ => return,
        };
        let message = match general_message {
            Standard(m) => m,
            _ => return,
        };
        let channel = message.channel.as_ref().unwrap();
        let message_text = message.text.as_ref().unwrap();
        // Make sure we only process a DM or a mention
        if !is_dm_channel(channel) && !message_contains_mention(&self.bot_id, message_text) {
            return;
        }
        // Check this message for name request
        let captures = TRANSLATE_NAME_REGEX.captures(message_text);
        let real_name = captures.and_then(|c| c.get(1)).map(|m| m.as_str());
        // Get the QNickname and form a response
        let resp_message_text: Cow<'static, str> = if let Some(real_name) = real_name {
            match get_qnickname(real_name) {
                Some(qnickname) => Cow::Owned(format!("Your QNickname is: *{}*", qnickname)),
                None => Cow::Borrowed("You don't have a QNickname! (Sorry!)"),
            }
        } else {
            Cow::Borrowed(
                "*Sorry I can't understand that!* :exploding_head:\n\
                 Try typing: \"Translate: [Your Real Name Here]\"",
            )
        };
        // Finally send the response out
        if let Err(error) = self.sender.send_message(channel, &resp_message_text) {
            let _ = writeln!(io::stderr(), "Error (Ignored): {}", error);
        }
    }
}

pub struct QNicknameBot {
    client: RtmClient,
    handler: QHandler,
}

impl QNicknameBot {
    pub fn login(api_token: &str) -> Result<QNicknameBot, slack::Error> {
        let bot_id = QNicknameBot::get_bot_id(&api_token)?
            .ok_or_else(|| slack::Error::Internal("No bot ID".to_string()))?;
        let client = RtmClient::login(api_token)?;
        let sender = client.sender().clone();
        let handler = QHandler { bot_id, sender };
        Ok(QNicknameBot { client, handler })
    }

    fn get_bot_id(api_token: &str) -> Result<Option<String>, slack::Error> {
        let client = slack::api::default_client()?;
        let response = slack::api::users::list(&client, api_token, &Default::default())
            .map_err(|_| slack::Error::Internal("Bad bot ID request".to_string()))?;
        let users = response.members.as_ref().unwrap();
        let qn_bot_users: Vec<&slack::User> = users
            .iter()
            .filter(|user| user.is_bot.unwrap() && user.name.as_ref().unwrap() == "qnicknamebot")
            .collect();
        match qn_bot_users.as_slice() {
            [qn_bot_user] => Ok(Some(qn_bot_user.id.as_ref().unwrap().to_string())),
            _ => Ok(None),
        }
    }

    pub fn run(&mut self) -> Result<(), slack::Error> {
        self.client.run(&mut self.handler)
    }
}
