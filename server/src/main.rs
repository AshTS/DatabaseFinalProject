#[macro_use] 
extern crate rocket;
use std::{path::{PathBuf}, sync::atomic::Ordering};

use mongodb::bson::oid::ObjectId;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::mongodb::*;

use rocket::{serde::json::Json, http::{Header}, Response, fairing::{Fairing, Info, Kind}, Request, State, futures::StreamExt};
use serde::{Serialize, Deserialize};

use std::sync::atomic::AtomicUsize;

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Spell {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    range: usize, // Feet
    duration: usize, // In Seconds
    requirements: ObjectId,
    level: u8
}


/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct SpellRequirement {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    verbal: bool,
    somatic: bool,
    material: bool
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Class {
    Bard,
    BlackGuard,
    Cleric,
    Druid,
    Paladin,
    Ranger,
    Wizard
}

/// Define a type that models our data.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct SpellClass {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    class: Class,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[get("/all")]
async fn read(mut db: Connection<DataStuff>) -> String {
    let mut items = db.database("local").collection::<SpellRequirement>("testdb").find(None, None).await.unwrap();

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

#[launch]
async fn rocket() -> _ {
    rocket::build().manage(HitCount { count: AtomicUsize::new(0) }).attach(DataStuff::init()).attach(Cors).mount("/", routes![index, count, read])
}