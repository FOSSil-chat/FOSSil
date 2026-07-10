use dioxus::prelude::*;
use dioxus_native::{Config, WindowAttributes};

pub fn launch() {
    let config =
        Config::new().with_window_attributes(WindowAttributes::default().with_title("FOSSil Chat"));

    dioxus_native::launch_cfg(App, vec![], vec![Box::new(config)]);
}

#[component]
fn App() -> Element {
    rsx! {
        h1 { "FOSSil Chat" }



    }
}
