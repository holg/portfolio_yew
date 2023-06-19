use crate::components::sidebar::Sidebar;
use crate::helpers::get_assets::get_page_assets;
use crate::helpers::get_assets::wrap;
use crate::helpers::get_assets::Asset;
use serde::Deserialize;
use yew::prelude::*;

// Used for parsing the data retrieved from remote server
// I was lazy and did not make vectors for projects, which
// I totally should have done.
#[derive(Deserialize, Debug, Default)]
pub struct CVData {
    name: String,
    phone: String,
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

pub struct CV {
    data: CVData,
}

pub enum Msg {
    Update(Result<String, reqwasm::Error>),
}

impl Component for CV {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // arrange for HTTP request to be made
        wasm_bindgen_futures::spawn_local(wrap(
            get_page_assets(Asset::CV),
            ctx.link().callback(Msg::Update),
        ));
        Self {
            data: CVData::default(),
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
                            log::error!("error parsing json in CV page: {:?}", err);
                            CVData::default()
                        }
                    };
                    true
                }
                Err(err) => {
                    log::error!("error retrieving CV page data: {:?}", err);
                    false
                }
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Sidebar/>
                <div class="cv-container">
                    <div class="cv">
                        <h1> { self.data.name.clone() } </h1>
                        <h2> { self.data.phone.clone() } </h2>
                        <hr class="divider"/>
                        <div class="section">
                            <h3> { "CV Experience" } </h3>
                            <p> { self.data.xpheader.clone() } </p>
                            <ul>
                                <li> { self.data.xpitem.clone() } </li>
                                <li> { self.data.xpitem2.clone() } </li>
                                <li> { self.data.xpitem3.clone() } </li>
                            </ul>
                        </div>
                        <hr class="divider"/>
                        <div class="section">
                            <h3> { "Education" } </h3>
                            <ul>
                                <li> { self.data.edheader.clone() } </li>
                                <li> { self.data.editem.clone() } </li>
                                <li> { self.data.editem2.clone() } </li>
                            </ul>
                        </div>
                        <hr class="divider"/>
                        <div class="section">
                            <h3> { "Projects and Skills" } </h3>
                            <p> { self.data.prheader.clone() } </p>
                            <ul>
                                <li> { self.data.pritem.clone() } </li>
                                <li> { self.data.pritem2.clone() } </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
