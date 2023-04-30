use gloo_net::http::Request;
use spells::Spell;
use yew::callback;
use yew::prelude::*;

use yew::function_component;
use super::InputElement;
use super::TagToggleInput;


#[function_component]
pub fn NewSpellComponent() -> Html {
    let state = use_state(Spell::default);

    let on_change_name = {
        let s = state.clone();
        move | text: String | {
            let mut spell = (*s).clone();
            spell.name = text;
            s.set(spell);
        }
    };

    let on_change_duration = {
        let s = state.clone();
        move | value: usize | {
            let mut spell = (*s).clone();
            spell.duration = value;
            s.set(spell);
        }
    };

    let on_change_range = {
        let s = state.clone();
        move | value: usize | {
            let mut spell = (*s).clone();
            spell.range = value;
            s.set(spell);
        }
    };

    let on_change_somatic = {
        let s: UseStateHandle<Spell> = state.clone();
        move | text: bool | {
            info!("Somatic: {}", text);
        }
    };

    let on_change_verbal = {
        let s: UseStateHandle<Spell> = state.clone();
        move | text: bool | {
            info!("Verbal: {}", text);
        }
    };

    let on_change_material = {
        let s: UseStateHandle<Spell> = state.clone();
        move | text: bool | {
            info!("Material: {}", text);
        }
    };

    let onclick = 
        callback::Callback::from(move | event: MouseEvent | {
            let s = state.clone();
            event.prevent_default();
            wasm_bindgen_futures::spawn_local(async move {
                let spell = serde_json::to_value(&*s).unwrap();
                if !s.name.is_empty() {
                    Request::post("http://localhost:8000/new").body(serde_json::to_string_pretty(&spell).unwrap()).send().await.unwrap();
                }
                // let fetched: Vec<Spell> = 
            });
            });

    html! {
        <div class="block box">
            <InputElement<String> name="Name" on_change={on_change_name} />
            <div class="columns">
                <div class="column is-three-quarters">
                    <InputElement<usize> name="Duration" on_change={on_change_duration} />
                    <InputElement<usize> name="Range" on_change={on_change_range} />
                </div>
                <div class="buttons column">
                    <br />
                    <TagToggleInput name="Somatic" color="is-success" on_change={on_change_somatic} />
                    <TagToggleInput name="Verbal" color="is-warning" on_change={on_change_verbal} />
                    <TagToggleInput name="Material" color="is-danger" on_change={on_change_material} />
                </div>
            </div>
            <div class="columns is-centered">
                <div class="column is-three-quarters">
                    <input class="button is-fullwidth is-normal is-primary" type="submit" {onclick} />
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn Thingy() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
            info!("{}", *counter);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}