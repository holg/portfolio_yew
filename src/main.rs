//! CS 410P Final Project: Professional Portfolio built with Yew
//!
//! Ross Morrison 2022
mod components;
mod helpers;
mod pages;
mod router;
use router::route_objects::*;
use yew::function_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

// It all starts here
fn main() {
    // enable console logging
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
