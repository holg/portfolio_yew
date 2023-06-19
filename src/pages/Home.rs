use crate::components::list::ListProjects;
use crate::components::project::Project;
use crate::components::sidebar::Sidebar;
use crate::helpers::get_assets::get_page_assets;
use crate::helpers::get_assets::wrap;
use crate::helpers::get_assets::Asset;
use serde::Deserialize;
use yew::prelude::*;

// Used for parsing the data retrieved from remote server
#[derive(Deserialize, Debug)]
struct ProjectData {
    img: String,
    title: String,
    desc: String,
    home_href: String,
    git_href: String,
}

// A collection of projects
#[derive(Deserialize, Debug)]
struct Projects {
    projects: Vec<ProjectData>,
}

impl Projects {
    fn new() -> Self {
        Self {
            projects: Vec::new(),
        }
    }
}

pub struct Home {
    data: Projects,
}

impl Home {
    pub fn new() -> Self {
        Self {
            data: Projects::new(),
        }
    }
}

pub enum Msg {
    Update(Result<String, reqwasm::Error>),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // arrange for HTTP request to be made
        wasm_bindgen_futures::spawn_local(wrap(
            get_page_assets(Asset::Home),
            ctx.link().callback(Msg::Update),
        ));
        Home::new()
    }

    // Process the results of our HTTP request. This means we need to do some
    // parsing or error handling.
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(response) => match response {
                // we were able to retrieve the response body
                Ok(doc) => {
                    self.data = match serde_json::from_str(&doc) {
                        Ok(data) => data,
                        Err(err) => {
                            log::error!("error parsing json in homepage: {:?}", err);
                            Projects::new()
                        }
                    };
                    true
                }
                // there must have been some sort of error.
                Err(err) => {
                    log::error!("error retrieving homepage data: {:?}", err);
                    false
                }
            },
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Each project must be mapped to an HTML component
        let projects = self.data.projects.iter().map(|prj| {
            html_nested! {
                <Project
                    img={prj.img.clone()}
                    home_link={prj.home_href.clone()}
                    git_link={prj.git_href.clone()}
                    title={prj.title.clone()}
                    description={prj.desc.clone()}
                />
            }
        });

        html! {
            <div>
                <Sidebar/>
                <div class="portfolio-container">
                    <img class= "image" src="./assets/home-designer.gif"/>
                    <h1 class="name">{ "Holger Trahe" }</h1>
                    <h2 class="credentials"> { "Software Developer" }</h2>
                    <hr class="divider"/>
                    <ListProjects>
                        {for projects}
                    </ListProjects>
                </div>
            </div>
        }
    }
}
