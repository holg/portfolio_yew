use reqwasm::http::Request;
use serde::Deserialize;
use std::result::Result;
pub enum Asset {
    Home,
    About,
    Resume,
    CV,
    Projects
}

// This function makes an HTTP request to the server for the content of
// whichever component made the request. The component signals this with the
// "page" argument. So the About page calls "get_page_assets(Asset::About)",
// and then receives a paragraph about me, the site author.
pub async fn get_page_assets(page: Asset) -> Result<String, reqwasm::Error> {
    // This struct allows us to extract the body from the HTTP response object
    #[derive(Deserialize, Debug)]
    struct HttpJson {
        body: serde_json::Value
    }
    match page {
        Asset::Home => match Request::get("/assets/home_assets.json")
            .send()
            .await
        {
            Ok(resp) => match resp.json().await {
                Ok(HttpJson { body }) => Ok(body.to_string()),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        },
        Asset::About => {
            match Request::get("/assets/about_assets.json")
                .send()
                .await
            {
                Ok(resp) => match resp.json().await {
                    Ok(HttpJson { body }) => Ok(body.to_string()),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            }
        }
        Asset::Resume => {
            match Request::get("/assets/resume_assets.json")
                .send()
                .await
            {
                Ok(resp) => match resp.json().await {
                    Ok(HttpJson { body }) => Ok(body.to_string()),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            }
        }
        Asset::CV => {
            match Request::get("/assets/cv_assets.json")
                .send()
                .await
            {
                Ok(resp) => match resp.json().await {
                    Ok(HttpJson { body }) => Ok(body.to_string()),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            }
        }
        Asset::Projects => {
            match Request::get("/assets/projects_assets.json")
                .send()
                .await
            {
                Ok(resp) => match resp.json().await {
                    Ok(HttpJson { body }) => Ok(body.to_string()),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            }
        }
    }
}

// This function tells the parent component that its assets are ready. The "done_cb"
// argument is a function that alerts the component that it needs to update, and "f"
// is "get_page_assets". So calling wrap generates a Future that will eventually tell
// a component that the data it requested from the server is ready.
pub async fn wrap<F: std::future::Future>(f: F, done_cb: yew::Callback<F::Output>) {
    done_cb.emit(f.await);
}
