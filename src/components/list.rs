use crate::components::project::Project;
use crate::components::station::Station;
use yew::prelude::*;

// The list can ONLY contain projects. No real logic behind this,
// Just wanted to see how the ChildrenWithProps type worked.
#[derive(Properties, PartialEq)]
pub struct ListProjectProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Project>,
}

pub struct ListProjects {}

impl Component for ListProjects {
    type Message = ();
    type Properties = ListProjectProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="list">
                {for ctx.props().children.iter()}
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ListStationProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Station>,
}

pub struct ListStations {}

impl Component for ListStations {
    type Message = ();
    type Properties = ListStationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="liststation">
                {for ctx.props().children.iter()}
            </div>
        }
    }
}
