use std::env;
use inline_python::{python, Context};
use pyo3::{prelude::*};

pub mod handlers;
use handlers::{help_message, request_url_information, get_useful_links, get_emails};

#[pyfunction]
fn message_handler(py: Python<'_>, message: String, user_id: i32, chat_id: i32, bot: Py<PyAny>) -> String {
    let Some(command) = message.as_str().split(" ").nth(0) else {
        return "No se ha encontrado el comando".to_string();
    };
    match command {
        "help" => help_message(user_id),
        "siu" => request_url_information(py, user_id,chat_id,message, bot),
        "calendar" | "calendario" | "feriados" => get_useful_links(user_id, message),
        "mails" => get_emails(user_id, message),
        _ => {
            println!("No se ha encontrado el comando");
            "No se ha encontrado el comando".to_string()
        },
    }
}

fn main() {
    // read environment variables
    dotenv::dotenv().ok();
    // Read the token from the environment variable
    let mode = env::var("MODE").unwrap_or_default();
    let token = env::var("TOKEN").expect("TOKEN must be set");
    let port = env::var("PORT").unwrap_or(5000.to_string()).parse::<i32>().unwrap();

	let c = Context::new();
	c.add_wrapped(wrap_pyfunction!(message_handler));

    c.run(
        python! {
            import telebot
            from flask import Flask, request
            import os
            import logging
            
            bot = telebot.TeleBot('token, parse_mode="HTML")

            def run():
               if 'mode == "prod":
                    server.run(host="0.0.0.0", port='port)
               else:
                    bot.polling()

            server = Flask(__name__)

            @bot.message_handler()
            def general_handler(message):
                content = message.text.replace('/', "").replace("@infoUNO_bot", "").replace("@UNOTestBots_BOT", "")
                user_id = message.from_user.id
                chat_id = message.chat.id
                result = message_handler(content, user_id, chat_id, bot)
                bot.send_message(chat_id, result)

            @server.route('/' + 'token, methods=["POST"])
            def getMessage():
                bot.process_new_updates(
                    [telebot.types.Update.de_json(request.stream.read().decode("utf-8"))])
                return "!", 200


            @server.route("/")
            def webhook():
                bot.remove_webhook()
                bot.set_webhook(url=os.environ.get("URL_NAME") + 'token)
                return "!", 200


            if __name__ == "__main__":
                run()

        }
    );

}