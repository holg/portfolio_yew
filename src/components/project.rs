use yew::prelude::*;

#[derive(Default)]
pub struct Project {}

// The information about a project will be passed as props
#[derive(Properties, PartialEq)]
pub struct ListProjectProps {
    pub img: String,
    pub home_link: String,
    pub git_link: String,
    pub title: String,
    pub description: String,
}

impl Component for Project {
    type Message = ();
    type Properties = ListProjectProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="card">
                <a href={ ctx.props().home_link.clone() }>
                    <img class ="project-image" src={ format!("./assets/{}", ctx.props().img.clone()) }/>
                </a>
                <a href={ ctx.props().git_link.clone() }>
                    <p class="project-title">{ ctx.props().title.clone() }</p>
                </a>
                <p class="project-description">{ctx.props().description.clone()}</p>
            </div>
        }
    }
}
