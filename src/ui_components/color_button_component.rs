use yew::prelude::*;
use crate::logic::colors::Colors;


pub struct ColorButtonComponent {
    content_text: String,
    class_text: String,
}

#[derive(Properties, PartialEq)]
pub struct ColorButtonProps {
    pub on_click: Callback<Colors>,
    pub is_enabled: bool,
    pub color: Colors
}

impl Component for ColorButtonComponent {
    type Message = ();
    type Properties = ColorButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        let color_text = ctx.props().color.to_string();
        let mut class_text = color_text.to_lowercase();
        class_text.push_str("_button");
        Self {
            content_text: color_text,
            class_text: class_text
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_click_callback = ctx.props().on_click.clone();
        let is_enabled = ctx.props().is_enabled;
        let color = ctx.props().color;
        html! {
            <button class={ &self.class_text }
            onclick={Callback::from(move |_| on_click_callback.emit(color))} 
            disabled={is_enabled}>{ &self.content_text }</button>
        }
    }
}