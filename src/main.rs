use sycamore::prelude::*;

#[component]
fn App() -> View {
    view! {
        h1 { "Hello from Rust + Sycamore + WebAssembly!" }
    }
}

fn main() {
    sycamore::render(App);
}
