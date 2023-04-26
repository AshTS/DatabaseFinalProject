#[macro_use] 
extern crate rocket;

use mongodb::bson::oid::ObjectId;
use rocket_db_pools::{Database, Connection};
use rocket::{serde::{json::Json}, http::{Header}, Response, fairing::{Fairing, Info, Kind}, Request, futures::StreamExt};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[get("/all")]
async fn read(mut db: Connection<DataStuff>) -> String {
    let mut items = db.database("local").collection::<spells::SpellRequirement>("testdb").find(None, None).await.unwrap();

    let mut data = String::new();

    while let Some(doc) = items.next().await {
        data += &format!("{:?}\n", doc);
    }

    data
}
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Database)]
#[database("database")]
struct DataStuff(rocket_db_pools::mongodb::Client);

#[get("/spell")]
fn produce_spell_data() -> Json<spells::Spell> {
    spells::Spell {
        name: "Spell".into(),
        range: 5,
        duration: 3600,
        level: 4
    }.into()
}

#[launch]
async fn rocket() -> _ {
    rocket::build().attach(DataStuff::init()).attach(Cors).mount("/", routes![index, read, produce_spell_data])
}