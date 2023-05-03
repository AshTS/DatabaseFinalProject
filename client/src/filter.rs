use spells::Spell;
use yew::prelude::*;

use crate::inputs::InputElement;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Filter {
    pub name: Option<String>,
    pub range_low: Option<usize>,
    pub range_high: Option<usize>,
    
    pub duration_low: Option<usize>,
    pub duration_high: Option<usize>
}

impl Filter {
    pub fn matches_spell(&self, spell: &Spell) -> bool {
        let mut pass: bool = true;

        if let Some(name) = &self.name {
            pass &= spell.name.to_lowercase().contains(&name.to_lowercase())
        }

        if let Some(range_low) = self.range_low {
            pass &= spell.range >= range_low
        }

        if let Some(range_high) = self.range_high {
            pass &= spell.range <= range_high
        }

        if let Some(duration_low) = self.duration_low {
            pass &= spell.duration >= duration_low
        }

        if let Some(duration_high) = self.duration_high {
            pass &= spell.duration <= duration_high
        }

        pass
    }
}

#[derive(Properties, PartialEq)]
pub struct FilterProperties {
    pub on_reload: Callback<Filter>
}

pub struct FilterComponent {
    filter: Filter
}

pub enum FilterUpdate {
    Name(Option<String>),
    RangeLow(Option<usize>),
    RangeHigh(Option<usize>),
    DurationLow(Option<usize>),
    DurationHigh(Option<usize>)
}

impl Component for FilterComponent {
    type Message = FilterUpdate;
    type Properties = FilterProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            filter: Filter {
                name: None,
                range_low: None,
                range_high: None,
                duration_low: None,
                duration_high: None,
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FilterUpdate::Name(name) => self.filter.name = name,
            FilterUpdate::RangeLow(range) => self.filter.range_low = range,
            FilterUpdate::RangeHigh(range) => self.filter.range_high = range,
            FilterUpdate::DurationLow(duration) => self.filter.duration_low = duration,
            FilterUpdate::DurationHigh(duration) => self.filter.duration_high = duration,
        };
        ctx.props().on_reload.emit(self.filter.clone());
        true
        
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change_name = ctx.link().batch_callback(|text: String| {
            Some(FilterUpdate::Name(Some(text)))
        });

        let on_change_range_low = ctx.link().batch_callback(|v: usize| {
            Some(FilterUpdate::RangeLow(Some(v)))
        });

        let on_change_range_high: Callback<usize> = ctx.link().batch_callback(|v: usize| {
            Some(FilterUpdate::RangeHigh(Some(v)))
        });

        let on_change_duration_low = ctx.link().batch_callback(|v: usize| {
            Some(FilterUpdate::DurationLow(Some(v)))
        });

        let on_change_duration_high: Callback<usize> = ctx.link().batch_callback(|v: usize| {
            Some(FilterUpdate::DurationHigh(Some(v)))
        });

        html! {
            <div class="box">
                <p class="title is-4">{"Filter"}</p>
                <div class="columns is-desktop is-multiline">
                    <div class="column"> 
                        <p>{"Name"}</p>
                        <InputElement<String> on_change={on_change_name}/>
                    </div>
                    
                    <div class="column"> 
                        <p>{"Range"}</p>
                        <div class="columns">
                            <div class="column"><InputElement<usize> on_change={on_change_range_low}/></div>
                            <div class="is-1 column"><p class="content is-medium" align="center">{ "-" }</p></div>
                            <div class="column"><InputElement<usize> on_change={on_change_range_high}/></div>
                        </div>
                    </div>
                    
                    <div class="column"> 
                        <p>{"Duration"}</p>
                        <div class="columns">
                            <div class="column"><InputElement<usize> on_change={on_change_duration_low}/></div>
                            <div class="is-1 column"><p class="content is-medium" align="center">{ "-" }</p></div>
                            <div class="column"><InputElement<usize> on_change={on_change_duration_high}/></div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}