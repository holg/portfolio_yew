# Professional-Portfolio: My personal portfolio website
## Instructions
For this project you must have trunk installed. Run
``` cargo install trunk ``` to install it. You'll also need to add the WASM build target by running ``` rustup target add wasm32-unknown-unknown```. Now clone the repo and run ```trunk serve --open```. This will open the project in your browser. The remote server is hosted at https://portfolio-server-rust-final.herokuapp.com/. You shouldn't need to worry about it, but in case it's not working for you, you can just go to my other repo in gitlab named "content-server" and follow the instructions there to run the server in your browser.

## Project Scope
The portfolio website built with Yew: https://yew.rs/. It hosts a list of projects I've worked on and some contact information/resume material. The goal was to create a good-looking website that runs well. This project was built with rust version 1.68.1 and yew 0.19

## Results
The Content is mainly served via JSON files, which are easy to extend and to maintain
Debugging and testing SSR, which did not interest me, as the Host server is a cheap 1€/month server,
which is not well suited for SSR.
https://trahe.eu


## Acknowledgments
# RossMorrison for the inspiration https://gitlab.cecs.pdx.edu/ros25/professional-portfolio
# yew.rs for the framework https://yew.rs/
# rust for the language https://www.rust-lang.org/

## License
This project is licensed under the MIT License

