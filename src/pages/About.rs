use crate::components::sidebar::Sidebar;
use crate::helpers::get_assets::get_page_assets;
use crate::helpers::get_assets::wrap;
use crate::helpers::get_assets::Asset;
use serde::Deserialize;
use yew::prelude::*;

// Used for parsing the data retrieved from remote server
#[derive(Deserialize, Debug, Default)]
pub struct AboutData {
    blurb: Vec<String>,
}

pub struct About {
    data: AboutData,
}

pub enum Msg {
    Update(Result<String, reqwasm::Error>),
}

impl Component for About {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // arrange for HTTP request to be made
        wasm_bindgen_futures::spawn_local(wrap(
            get_page_assets(Asset::About),
            ctx.link().callback(Msg::Update),
        ));
        Self {
            data: AboutData::default(),
        }
    }

    // Process the results of our HTTP request. This means we need to do some
    // parsing or error handling.
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(response) => match response {
                Ok(doc) => {
                    self.data = match serde_json::from_str(&doc) {
                        Ok(data) => data,
                        Err(err) => {
                            log::error!("error parsing json in about page: {:?}", err);
                            AboutData::default()
                        }
                    };
                    true
                }
                Err(err) => {
                    log::error!("error retrieving about page data: {:?}", err);
                    false
                }
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let paragraphs = self.data.blurb.iter().map(|par |{
            html_nested! {
                <p> { par.clone() } </p>
            }
        });

        html! {
            <div>
                <Sidebar/>
                <div class="about-container">
                    <img class="avatar" src="./assets/avatar.jpg"/>
                    <h1> { "About me" } </h1>
                    <div class="about-box">
                       <p> { for paragraphs } </p>
                    </div>
                <img class="avatar" src="./assets/avatar2.jpg"/>
                </div>
            </div>
        }
    }
}
