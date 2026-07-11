use dioxus::prelude::*;
use dioxus_native::{Config, WindowAttributes};

const REGULAR: Asset = asset!("/assets/fonts/LiberationSans-Regular.ttf");
const BOLD: Asset = asset!("/assets/fonts/LiberationSans-Bold.ttf");
const BG_IMAGE: Asset = asset!("assets/bg.png");

pub fn launch() {
    let config =
        Config::new().with_window_attributes(WindowAttributes::default().with_title("FOSSil Chat"));

    dioxus_native::launch_cfg(App, vec![], vec![Box::new(config)]);
}

#[component]
fn App() -> Element {
        let fonts = format!(
        "@font-face {{
            font-family: 'Liberation Sans';
            src: url('{REGULAR}') format('truetype');
            font-weight: normal;
        }}
        @font-face {{
            font-family: 'Liberation Sans';
            src: url('{BOLD}') format('truetype');
            font-weight: bold;
        }}"
    );

    rsx! {

        style { "{fonts}" }

        div {

            style: "position: relative; width: 100vw; height: 100vh; overflow: hidden;",

            img {
                src: "{BG_IMAGE}",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; object-fit: fill;"
            }
            
            div {

                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%;
                         display: flex; flex-direction: column;
                         justify-content: center; align-items: flex-end;
                         gap: 2vh; padding: 4vw; box-sizing: border-box;", 
                p {
                    style: "font-family: 'Liberation Sans'; font-weight: bold; font-size: 2.05vw;",
                    "FOSSil Chat"
                }

                p {
                    style: "font-family: 'Liberation Sans'; font-weight: normal; font-size: 1.25vw;",
                    "Welcome to FOSSil Chat!"
                }

            }


        }



    }
}
