use std::marker::PhantomData;

use web_sys::HtmlInputElement;
use yew::prelude::*;

macro_rules! implement_for_type {
    ($t: ty, $tag: expr) => {
        #[allow(clippy::redundant_closure_call)]
        impl Component for InputElement<$t> {
            type Message = Msg<$t>;
            type Properties = InputProperties<$t>;
        
            fn create(_ctx: &Context<Self>) -> Self {
                Self::default()
            }
        
            fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
                match msg {
                    Msg::Update(value) => {
                        ctx.props().on_change.emit(value);
                        true
                    }
                }
            }
        
            fn view(&self, ctx: &Context<Self>) -> Html {
                let onchange = ctx.link().batch_callback(|e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    let value = input.value().parse().unwrap_or_default();
                    Some(Msg::Update(value))
                });
                html! {
                    <>
                    if let Some(name) = &ctx.props().name {
                        <p>{ name }</p>
                    }
                    { $tag(onchange) }
                    </>
                }
            }
        }
    };
    (raw $t: ty, $tag: expr) => {
        #[allow(clippy::redundant_closure_call)]
        impl Component for InputElement<$t> {
            type Message = Msg<$t>;
            type Properties = InputProperties<$t>;
        
            fn create(_ctx: &Context<Self>) -> Self {
                Self::default()
            }
        
            fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
                match msg {
                    Msg::Update(value) => {
                        ctx.props().on_change.emit(value);
                        true
                    }
                }
            }
        
            fn view(&self, ctx: &Context<Self>) -> Html {
                let onchange = ctx.link().batch_callback(|e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    let value = input.value().parse().unwrap_or_default();
                    Some(Msg::Update(value))
                });
                $tag(onchange, ctx)
            }
        }
    };
}

/// The `Parent` component holds some state that is updated when its children are clicked
#[derive(Debug, Default)]
pub struct InputElement<T: Default> {
    _phantom: PhantomData<T>
}

#[derive(Properties, PartialEq)]
pub struct InputProperties<T: PartialEq> {
    pub on_change: Callback<T>,
    pub name: Option<String>
}

#[derive(Debug, Clone)]
pub enum Msg<T: std::fmt::Debug + Clone> {
    Update(T),
}

#[derive(Debug, Default)]
pub struct TagToggleInput {
    value: bool
}

#[derive(Properties, PartialEq)]
pub struct TagToggleInputProperties {
    pub on_change: Callback<bool>,
    pub name: String,
    pub color: Option<String>
}

implement_for_type!(String, (|onchange| 
    html! {
        <input type="text" class="input is-normal is-primary"
                {onchange}
            />
    }));

implement_for_type!(usize, (|onchange| 
    html! {
        <input type="number" class="input is-normal is-primary" min="0" max="16777216"
            {onchange}
        />
    }));

implement_for_type!(u8, (|onchange| 
    html! {
        <input type="number" class="input is-normal is-primary" min="0" max="256"
            {onchange}
        />
    }));

implement_for_type!(raw bool, (|onchange, ctx: &Context<Self> | 
    html! {
        <div class="checkbox">
            <input type="checkbox"
                    {onchange}
                />
            if let Some(name) = &ctx.props().name {
                { name }
            }
        </div>
    }));

impl Component for TagToggleInput {
    type Message = Msg<()>;
    type Properties = TagToggleInputProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(_) => {
                self.value = !self.value;
                ctx.props().on_change.emit(self.value);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().batch_callback(|e: MouseEvent| {
            Some(Msg::Update(()))
        });

        let color = if let Some(color) = &ctx.props().color { color.clone() } else { "is-danger".into() };
        let text = if self.value { "" } else { "is-light" };
        let classes = format!("button is-normal is-fullwidth mb-5 {} {}", color, text);

        html! {
            <button class={ classes } {onclick}>{ &ctx.props().name }</button>
        }
    }
}