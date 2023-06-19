use yew::prelude::*;

#[derive(Default)]
pub struct Station {}

#[derive(Properties, PartialEq)]
pub struct ListStationProps {
    pub xpheader: String,
    pub xpitem: String,
    pub xpitem2: String,
    pub xpitem3: String,
    pub edheader: String,
    pub editem: String,
    pub editem2: String,
    pub prheader: String,
    pub pritem: String,
    pub pritem2: String,
}
impl Component for Station {
    type Message = ();
    type Properties = ListStationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="station">
                <h3>{ctx.props().xpheader.clone()}</h3>
                <p>{ctx.props().xpitem.clone()}</p>
                <p>{ctx.props().xpitem2.clone()}</p>
                <p>{ctx.props().xpitem3.clone()}</p>
                <p>{ctx.props().edheader.clone()}</p>
                <p>{ctx.props().editem.clone()}</p>
                <p>{ctx.props().editem2.clone()}</p>
                <p>{ctx.props().prheader.clone()}</p>
                <p>{ctx.props().pritem.clone()}</p>
                <p>{ctx.props().pritem2.clone()}</p>
                <hr class="divider"/>
            </div>
        }
    }
}
