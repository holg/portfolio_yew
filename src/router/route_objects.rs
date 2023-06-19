use crate::pages::about::About;
use crate::pages::home::Home;
use crate::pages::resume::Resume;
use crate::pages::cv::CV;
use crate::pages::projects::Projects;
use yew::prelude::*;
use yew_router::prelude::*;

// All possible routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/resume")]
    Resume,
    #[at("/cv")]
    CV,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Switch function, used in main.rs to render routes depending on the current URL
pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home/>
        },
        Route::About => html! {
            <About/>
        },
        Route::Resume => html! {
            <Resume/>
        },
        Route::CV => html! {
            <CV/>
        },
        Route::Projects => html! {
            <Projects/>
        },
        Route::NotFound => html! {
            <h1>{ "404" }</h1>
        },
    }
}
