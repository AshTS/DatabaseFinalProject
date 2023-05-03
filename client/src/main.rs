#[macro_use]
extern crate log;

use gloo_net::http::Request;
use spells::{Spell, SpellPair, ClassLevelPair};
use yew::prelude::*;

mod filter;
mod form;
mod inputs;
use inputs::*;
use filter::*;

use form::{NewSpellComponent};

#[derive(Properties, PartialEq)]
struct SpellListProps {
    spells: Vec<SpellPair>,
}

fn render_range(range: usize) -> String {
    if range == 0 {
        "Touch".into()
    }
    else if range > 1024 * 1024 {
        "Infinite".into()
    }
    else if range >= 5280 && range % 5280 == 0 {
        format!("{} mile{}", range / 5280, if range / 5280 > 1 { "s" } else { "" })
    }
    else {
        format!("{} {}", range, if range > 1 { "feet" } else { "foot" })
    }
}

fn render_duration(duration: usize) -> String {
    if duration == 6 {
        "1 round".to_string()
    }
    else if duration == 0 {
        "Instant".to_string()
    }
    else if duration >= (24 * 3600) && duration % (24 * 3600) == 0 {
        format!("{} day{}", duration / (24 * 3600), if duration / (24 * 3600) > 1 { "s" } else { "" })
    }
    else if duration >= 3600 && duration % 3600 == 0 {
        format!("{} hour{}", duration / 3600, if duration / 3600 > 1 { "s" } else { "" })
    }
    else if duration >= 60 && duration % 60 == 0 {
        format!("{} minute{}", duration / 60, if duration / 60 > 1 { "s" } else { "" })
    }
    else {
        format!("{} second{}", duration, if duration > 1 { "s" } else { "" })
    }
}

#[function_component]
fn Spells(properties: &SpellListProps) -> Html {
    html!{
        <>
        <p class="ml-4">{format!("{} result{}", properties.spells.len(), if properties.spells.len() == 1 { "" } else { "s" })}</p>
        {
            properties.spells.iter().map(|spell| html! {
                <div class="card">
                    <div class="card-content">
                        <div class="columns">
                            <div class="column is-one-fifth columns">
                                <div class="column is-9"><p class="title is-4">{ &spell.spell.name } </p></div>
                                <div class="column">
                                    if spell.spell.area {
                                        <span class="tag">{"Area"}</span>
                                    }
                                </div>
                                <div class="column is-1"></div>
                            </div>
                            <div class="column">
                                <p> {format!("Range: {}", render_range(spell.spell.range))} </p>
                                <p> {format!("Duration: {}", render_duration(spell.spell.duration))} </p>
                            </div>
                            <div class="column field is-grouped is-grouped-multiline">
                                { spell.pairs.iter().map(|pair| html! {
                                    <div class="control">
                                        <div class="tags has-addons">
                                            <span class={format!("tag is-{}", pair.class.color())}>{pair.class.to_string()}</span>
                                            <span class="tag">{pair.level}</span>
                                        </div>
                                    </div>
                                }).collect::<Html>() }
                            </div>
                            <div class="column">
                                <div class="tags are-medium">
                                    <span class={ if !spell.spell.somatic { "tag is-success is-light" } else { "tag is-success" } }>{ "Somatic" }</span>
                                    <span class={ if !spell.spell.verbal { "tag is-warning is-light" } else { "tag is-warning" } }>{" Verbal" }</span>
                                    <span class={ if !spell.spell.material { "tag is-danger is-light" } else { "tag is-danger" } }>{ "Material" }</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            }).collect::<Html>()
        }
        </>
    }
    
}

#[function_component]
fn ServerSpell() -> Html {
    let spells = use_state(Vec::new);

    let s = spells.clone();
    use_effect_with_deps(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let fetched: Vec<SpellPair> = Request::get("http://localhost:8000/all").send().await.unwrap().json().await.unwrap();
            s.set(fetched);
        })
    }, ());

    let on_reload = {
        let s = spells.clone();
        move | filter: Filter | {
            info!("{:?}", filter);
            let t = s.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched: Vec<SpellPair> = Request::get("http://localhost:8000/all").send().await.unwrap().json().await.unwrap();
                t.clone().set(fetched.into_iter().filter(|spell| filter.matches_spell(&spell.spell)).collect());
            });
        }
    };

    html! {
        <> 
        <FilterComponent on_reload={on_reload} />
        <Spells spells={(*spells).clone()} />
        </>
    }
}


#[function_component]
fn App() -> Html {
    let show_add = use_state(|| false);

    let onclick = {
        let s = show_add.clone();
        move | _: MouseEvent | {
            s.set(!*s);
        }
    };

    html! {
        <div>
            <h1 class="is-size-2">{ "Database Final Project" }</h1>
            <article class="message">
                <div class="message-header" {onclick}>
                    <p>{"Add New"}</p>
                </div>
                if *show_add {
                    <div class="message-body">
                        <NewSpellComponent />
                    </div>
                }
            </article>
            <ServerSpell />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
