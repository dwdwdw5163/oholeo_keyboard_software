mod app;
mod component;
mod keyboard;

use app::App;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}


