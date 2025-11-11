use sycamore::prelude::*;

#[component]
fn App() -> View {
    view! {
        h1 { "Hello from Rust + Sycamore + WebAssembly!" }
        p { "This is a test." }
    }
}

fn main() {
    sycamore::render(App);
}
