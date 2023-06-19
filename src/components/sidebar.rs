use crate::router::route_objects::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

pub enum Msg {
    Toggle,
}

// is_open tells the component if it should show the sidebar
pub struct Sidebar {
    is_open: bool,
}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { is_open: false }
    }

    // update handles messages about the sidebar. The button sends these messages.
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match self.is_open {
            true => {
                self.is_open = false;
                true
            }
            false => {
                self.is_open = true;
                true
            }
        }
    }

    // contains the button that sends messages to "update." Changes view
    // based on is_open property.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let toggle = ctx.link().callback(|_| Msg::Toggle);

        if self.is_open {
            html! {
                <div class="sidebar-open">
                    <button class="btn" onclick={toggle}>
                        <img class= "sidebar-btn" src="./assets/exit.svg"/>
                    </button>
                    <div class="links">
                        <Link<Route> to={Route::Home}><img class="img-icon" src="./assets/home.jpg" />{ "Home" }</Link<Route>>
                        <Link<Route> to={Route::About}><img class="img-icon" src="./assets/about.jpg" />{ "About" }</Link<Route>>
                        <Link<Route> to={Route::Resume}><img class="img-icon" src="./assets/resume.jpg" />{ "Resume" }</Link<Route>>
                        // <Link<Route> to={Route::CV}>{ "CV" }</Link<Route>>
                        // <Link<Route> to={Route::Projects}>{ "Projects" }</Link<Route>>
                    </div>
                </div>
            }
        } else {
            html! {
                <>
                <button class="btn" onclick={toggle}>
                    <img class= "sidebar-btn" src="./assets/menu_btn.svg"/>
                </button>
                </>
            }
        }
    }
}
