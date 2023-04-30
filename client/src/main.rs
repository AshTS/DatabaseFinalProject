#[macro_use]
extern crate log;

use gloo_net::http::Request;
use spells::Spell;
use yew::prelude::*;

mod form;
mod inputs;
use inputs::*;

use form::{NewSpellComponent};

#[derive(Properties, PartialEq)]
struct SpellListProps {
    spells: Vec<Spell>,
}

#[function_component]
fn Spells(properties: &SpellListProps) -> Html {
    properties.spells.iter().map(|spell| html! {
        <div class="card">
            <div class="card-content">
                <div class="columns">
                    <div class="column is-one-fifth">
                        <p class="title is-4">{ &spell.name }</p>
                    </div>
                    <div class="column">
                        <p> {format!("Range: {}, Duration: {}", spell.range, spell.duration)} </p>
                    </div>
                    <div class="column">
                        <div class="tags are-medium">
                            <span class={ if !spell.somatic { "tag is-success is-light" } else { "tag is-success" } }>{ "Somatic" }</span>
                            <span class={ if !spell.verbal { "tag is-warning is-light" } else { "tag is-warning" } }>{" Verbal" }</span>
                            <span class={ if !spell.material { "tag is-danger is-light" } else { "tag is-danger" } }>{ "Material" }</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }).collect()
}

#[function_component]
fn ServerSpell() -> Html {
    let spells = use_state(Vec::new);
    {
        let spells = spells.clone();
        use_effect_with_deps(move |_| {
            let spells = spells;
            wasm_bindgen_futures::spawn_local(async move {
                let fetched: Vec<Spell> = Request::get("http://localhost:8000/all").send().await.unwrap().json().await.unwrap();
                spells.set(fetched);
            })
        }, ());
    }
    html! {
        <> <Spells spells={(*spells).clone()}> </Spells> </>
    }
}


#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1 class="is-size-2">{ "Database Final Project" }</h1>
            <NewSpellComponent />
            <ServerSpell />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
