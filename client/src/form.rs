use gloo_net::http::Request;
use spells::Class;
use spells::ClassLevelPair;
use spells::Spell;
use spells::SpellPair;
use yew::callback;
use yew::prelude::*;

use yew::function_component;
use crate::inputs::InputProperties;

use super::InputElement;
use super::TagToggleInput;


#[function_component]
pub fn NewSpellComponent() -> Html {
    let state = use_state(SpellPair::default);

    let on_change_name = {
        let s = state.clone();
        move | text: String | {
            let mut spell = (*s).clone();
            spell.spell.name = text;
            s.set(spell);
        }
    };

    let on_change_duration = {
        let s = state.clone();
        move | value: usize | {
            let mut spell = (*s).clone();
            spell.spell.duration = value;
            s.set(spell);
        }
    };

    let on_change_range = {
        let s = state.clone();
        move | value: usize | {
            let mut spell = (*s).clone();
            spell.spell.range = value;
            s.set(spell);
        }
    };

    let on_change_somatic = {
        let s = state.clone();
        move | value: bool | {
            let mut spell = (*s).clone();
            spell.spell.somatic = value;
            s.set(spell);
        }
    };

    let on_change_verbal = {
        let s = state.clone();
        move | value: bool | {
            let mut spell = (*s).clone();
            spell.spell.verbal = value;
            s.set(spell);
        }
    };

    let on_change_material = {
        let s = state.clone();
        move | value: bool | {
            let mut spell = (*s).clone();
            spell.spell.material = value;
            s.set(spell);
        }
    };
    
    let on_change_area = {
        let s = state.clone();
        move | value: bool | {
            let mut spell = (*s).clone();
            spell.spell.area = value;
            s.set(spell);
        }
    };

    let add_pair = {
        let s = state.clone();
        move | value: ClassLevelPair | {
            let mut spell = (*s).clone();
            spell.pairs.push(value);
            s.set(spell);
        }
    };

    let s = state.clone();
    let onclick = 
        callback::Callback::from(move | event: MouseEvent | {
            let s = s.clone();
            event.prevent_default();
            wasm_bindgen_futures::spawn_local(async move {
                let spell = serde_json::to_value(&*s).unwrap();
                if !s.spell.name.is_empty() {
                    Request::post("http://localhost:8000/new").body(serde_json::to_string_pretty(&spell).unwrap()).send().await.unwrap();
                }
            });
            });

    html! {
        <div class="block">
            <InputElement<String> name="Name" on_change={on_change_name} />
            <div class="columns">
                <div class="column is-three-quarters">
                    <InputElement<usize> name="Duration (seconds)" on_change={on_change_duration} />
                    <InputElement<usize> name="Range (feet)" on_change={on_change_range} />
                    <br />
                    <ClassLevelSubmit on_change={add_pair} />
                    <div class="column field is-grouped is-grouped-multiline">
                        { (*state).clone().pairs.iter().map(|pair| html! {
                            <div class="control">
                                <div class="tags has-addons">
                                    <span class={format!("tag is-{}", pair.class.color())}>{pair.class.to_string()}</span>
                                    <span class="tag">{pair.level}</span>
                                </div>
                            </div>
                        }).collect::<Html>() }
                    </div>
                </div>
                <div class="buttons column">
                    <br />
                    <TagToggleInput name="Somatic" color="is-success" on_change={on_change_somatic} />
                    <TagToggleInput name="Verbal" color="is-warning" on_change={on_change_verbal} />
                    <TagToggleInput name="Material" color="is-danger" on_change={on_change_material} />
                    <TagToggleInput name="Area" color="is-info" on_change={on_change_area} />
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

pub enum ClassLevelSubmitMessage {
    Class(Class),
    Level(u8),
    Submit
}

#[derive(Default)]
pub struct ClassLevelSubmit(ClassLevelPair);

impl Component for ClassLevelSubmit {
    type Message = ClassLevelSubmitMessage;
    type Properties = InputProperties<ClassLevelPair>;

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ClassLevelSubmitMessage::Class(c) => {self.0.class = c;},
            ClassLevelSubmitMessage::Level(l) => {self.0.level = l;},
            ClassLevelSubmitMessage::Submit => { ctx.props().on_change.emit(self.0.clone()); },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change_class = ctx.link().batch_callback(|e: Class| {
            Some(ClassLevelSubmitMessage::Class(e))
        });
        let on_change_level = ctx.link().batch_callback(|l: usize| {
            Some(ClassLevelSubmitMessage::Level((l & 255) as u8))
        });
        let onclick = ctx.link().batch_callback(|e: MouseEvent| {
            Some(ClassLevelSubmitMessage::Submit)
        });

        html! {
            <div class="columns">
                <div class="column is-2">
                    <InputElement<Class> name="Class" on_change={on_change_class} />
                </div>
                <div class="column">
                    <InputElement<usize> name="Level" on_change={on_change_level} />
                </div>
                <div class="column is-1">
                    <input class="button is-normal is-primary mt-5" type="submit" value="+" {onclick} />
                </div>
            </div>   
        }
    }
}
