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
            font-family: 'Liberation Sans Regular';
            src: url('{REGULAR}') format('truetype');
            font-weight: normal;
        }}
        @font-face {{
            font-family: 'Liberation Sans';
            src: url('{BOLD}') format('truetype');
            font-weight: bold;
        }}"

    );

    let rsx_style = format!(
        "width: 100%;
         height: 100vh;
         background-image: url('{BG_IMAGE}');
         background-size: cover;
         background-position: center;
         background-repeat: no-repeat;"
    );

    rsx! {

        style { "{fonts}" }

        div { 

            style: "{rsx_style}",            

            h1 { 
                style: "font-family: 'Liberation Sans'; font-weight: bold;",
                "FOSSil Chat" 
            }

            p { 
                style: "font-family: 'Liberation Sans'; font-weight: normal;",
                "Welcome to FOSSil Chat!" 
            }
            

            

        }



    }
}
