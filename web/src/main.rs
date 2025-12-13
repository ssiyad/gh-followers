use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Foo />
        <Bar />
        <Baz />
    }
}

#[component]
fn Foo() -> impl IntoView {
    view! {
        <div>"This is the Foo component."</div>
    }
}

#[component]
fn Bar() -> impl IntoView {
    view! {
        <div>"This is the Bar component."</div>
    }
}

#[component]
fn Baz() -> impl IntoView {
    view! {
        <div>"This is the Baz component."</div>
    }
}
