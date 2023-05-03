#[macro_use] 
extern crate rocket;

use std::collections::HashMap;

use rocket_db_pools::{Database, Connection};
use rocket::{serde::{json::Json}, http::{Header}, Response, fairing::{Fairing, Info, Kind}, Request, futures::StreamExt};
use spells::{Spell, ClassLevelPair, SpellPair};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

async fn load_class_level_pairs(db: &Connection<DataStuff>) -> Vec<ClassLevelPair> {
    let mut items = db.database("local").collection::<spells::ClassLevelPair>("class_table").find(None, None).await.unwrap();

    let mut data = vec![];
    while let Some(doc) = items.next().await {
        if let Ok(entry) = doc {
            data.push(entry);
        }
    }

    data
}

async fn load_spells(db: &Connection<DataStuff>) -> Vec<Spell> {
    let mut items = db.database("local").collection::<spells::Spell>("spell_table").find(None, None).await.unwrap();

    let mut data = vec![];

    while let Some(doc) = items.next().await {
        if let Ok(entry) = doc {
            data.push(entry);
        }
    }

    data
}


#[get("/all")]
async fn read(db: Connection<DataStuff>) -> Json<Vec<SpellPair>> {
    let mut spells: HashMap<usize, SpellPair> = load_spells(&db).await.into_iter().map(|spell| (spell.spell_id, SpellPair{ spell, pairs: vec![] })).collect();
    let class_level_pairs = load_class_level_pairs(&db).await;

    for class_level_pair in class_level_pairs {
        if let Some(pair) = spells.get_mut(&class_level_pair.spell_id) {
            pair.pairs.push(class_level_pair);
        }
    }

    spells.into_iter().map(|(_, v)| v).collect::<Vec<SpellPair>>().into()
}

#[post("/new", data = "<input>")]
async fn add(db: Connection<DataStuff>, mut input: Json<SpellPair>) -> Result<String, String> {
    let next_spell_id = load_spells(&db).await.into_iter().map(|s| s.spell_id).max().unwrap_or(0) + 1;

    input.0.spell.spell_id = next_spell_id;

    let SpellPair { spell, pairs } = input.0;

    let id = match db.database("local").collection::<spells::Spell>("spell_table").insert_one(spell, None).await {
        Ok(result) => Ok(result.inserted_id.to_string()),
        Err(e) => Err(e.to_string()),
    }?;

    for mut pair in pairs {
        pair.spell_id = next_spell_id;
        match db.database("local").collection::<spells::ClassLevelPair>("class_table").insert_one(pair, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }?;
    }

    Ok(id)
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