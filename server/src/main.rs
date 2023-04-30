#[macro_use] 
extern crate rocket;

use mongodb::bson::oid::ObjectId;
use rocket_db_pools::{Database, Connection};
use rocket::{serde::{json::Json}, http::{Header}, Response, fairing::{Fairing, Info, Kind}, Request, futures::StreamExt, Data};
use spells::Spell;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[get("/all")]
async fn read(mut db: Connection<DataStuff>) -> Json<Vec<Spell>> {
    let mut items = db.database("local").collection::<spells::Spell>("testdb").find(None, None).await.unwrap();

    let mut data = vec![];

    while let Some(doc) = items.next().await {
        if let Ok(entry) = doc {
            data.push(entry);
        }
    }

    data.into()
}

#[post("/new", data = "<input>")]
async fn add(mut db: Connection<DataStuff>, input: Json<Spell>) -> Result<String, String> {
    match db.database("local").collection::<spells::Spell>("testdb").insert_one(input.0, None).await {
        Ok(result) => Ok(result.inserted_id.to_string()),
        Err(e) => Err(e.to_string()),
    }
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

#[launch]
async fn rocket() -> _ {
    rocket::build().attach(DataStuff::init()).attach(Cors).mount("/", routes![index, read, add])
}