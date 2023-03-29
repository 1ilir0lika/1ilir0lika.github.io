#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket::form::Form;
use rocket::fs::NamedFile;
use teloxide::prelude::*;
use rocket::fs::relative;
#[derive(FromForm)]
struct Segnalazione {
    segnalazioni: String,
}


#[post("/segnalazioni.html", data = "<form>")]
async fn submit_segnalazione(form: Form<Segnalazione>) -> Option<NamedFile>{
    let segnalazione = form.into_inner();
    //mandare segnalazione a https://t.me/demorme_segnalazioni_bot
    //pretty_env_logger::init();
    let  chat_id:ChatId = ChatId(1755296389);
    let bot = Bot::from_env();
    bot.send_message(chat_id, segnalazione.segnalazioni).await.ok();
    //log::info!("segnalazione inviata");
    //aprire schermata bella di ringraziamento e mandare messaggio al bot telegram
    NamedFile::open(relative!("static/success.html")).await.ok()
}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![submit_segnalazione])
    .mount("/",FileServer::from("/home/demor/progetti/site/demorme"))
}