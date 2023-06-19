use crate::components::list::ListStations;
use crate::components::sidebar::Sidebar;
use crate::components::station::Station;
use crate::helpers::get_assets::get_page_assets;
use crate::helpers::get_assets::wrap;
use crate::helpers::get_assets::Asset;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Deserialize, Debug, Default)]
pub struct StationData {
    xpheader: String,
    xpitem: String,
    xpitem2: String,
    xpitem3: String,
    edheader: String,
    editem: String,
    editem2: String,
    prheader: String,
    pritem: String,
    pritem2: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Stations {
    stations: Vec<StationData>,
}

impl Stations {
    fn new() -> Self {
        Self {
            stations:Vec::new(),
        }
    }
}
#[derive(Deserialize, Debug, Default)]
pub struct ResumeData {
    name: String,
    phone: String,
    email: String,
    data: Stations,
}

pub struct Resume {
    data: Stations,
}

impl Resume {
    pub fn new() -> Self {
        Self {
            data: Stations::new(),
        }
    }
}

pub enum Msg {
    Update(Result<String, reqwasm::Error>),
}

impl Component for Resume {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // arrange for HTTP request to be made
        wasm_bindgen_futures::spawn_local(wrap(
            get_page_assets(Asset::Resume),
            ctx.link().callback(Msg::Update),
        ));
        Resume::new()
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
                            log::error!("error parsing json in resume page: {:?}", err);
                            Stations::new()
                        }
                    };
                    true
                }
                Err(err) => {
                    log::error!("error retrieving resume page data: {:?}", err);
                    false
                }
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Each station must be mapped to the HTML Element
        let stations = self.data.stations.iter().map(|stt |{
            html_nested! {
                <Station
                    xpheader= { stt.xpheader.clone() }
                    xpitem= { stt.xpitem.clone() }
                    xpitem2= { stt.xpitem2.clone() }
                    xpitem3= { stt.xpitem3.clone() }
                    edheader= { stt.edheader.clone() }
                    editem= { stt.editem.clone() }
                    editem2= { stt.editem2.clone() }
                    prheader= { stt.prheader.clone() }
                    pritem= { stt.pritem.clone() }
                    pritem2= { stt.pritem2.clone() }
                />
            }
        });

        html! {
            <div>
                <Sidebar/>
                <div class="resume-container">
                    <div class="resume">
                        <h1> {"Holger Trahe"} </h1>
                        <h2> { "trahe@mac.com" } </h2>
                        <h2> { "Open for new Opportunities" } </h2>
                        <h3> { "Work Experience" } </h3>
                            <div class="section">
                            <hr class="divider"/>
                        <ListStations>
                                { for stations }

                            </ListStations>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
