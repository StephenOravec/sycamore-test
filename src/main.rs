use sycamore::prelude::*;
use sycamore::web::Html;

#[component]
fn App<G: Html>() -> View<G> {
    view! {
        h1 { "Hello from Rust + Sycamore + WebAssembly!" }
    }
}

fn main() {
    sycamore::render(App);
}
