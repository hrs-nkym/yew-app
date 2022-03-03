use yew::prelude::*;
use components::header::Header;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Header />
        // <div>
        //     <h1>{"Hello, world!"}</h1>
        //     <button>{"Click me"}</button>
        // </div>
    }
}

fn main() {
    yew::start_app::<App>();
    // println!("Hello, world!");
}
