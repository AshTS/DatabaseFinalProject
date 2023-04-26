use yew::prelude::*;

#[function_component]
fn Render() -> Html {
    let spells = spells::get_sample_spells();
    spells.iter().map(|spell| html! {
        <><h2>{ &spell.name }</h2><p> {format!("Range: {}, Duration: {}, Level: {}", spell.range, spell.duration, spell.level)} </p></>
    }).collect()
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{ "Database Final Project" }</h1>
            <Render />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
