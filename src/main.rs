extern crate telegram_bot;
use telegram_bot::*;

pub static BOT_TOKEN: &'static str = "203403427:AAGbiRr7IbsN5WdVoPcgqMP9a_daW4yGkJM";

// NOTE: This is currently pretty much exactly the example as shown in the documentation of the
// telegram-bot api for rust.
// XXX: When the real bot is tried later on, the BOT_TOKEN must be hidden from the public repo,
// otherwise every user would be able to access the bot.
pub fn main() {
	let api = Api::from_token(BOT_TOKEN).expect("Api could not be engaged.");

	println!("getMe: {:?}", api.get_me());
	let mut listener = api.listener(ListeningMethod::LongPoll(None));

	listener.listen(|u| {
		// Does the received update contain a message.
		if let Some(m) = u.message {
			// Answer the message, in case it was a text message.
			if let MessageType::Text(text) = m.msg {
				// Answer message with "Hi"
				println!("{:?}", try!(api.send_message(
					m.chat.id(),
					format!("Hi, {}! d: {}", m.from.first_name, text),
					None, None, None, None
				)));
			}
		}

		Ok(ListeningAction::Continue)
	});
}
