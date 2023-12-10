mod app;
mod component;
mod keyboard;
mod bootstrap;

use app::App;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}


