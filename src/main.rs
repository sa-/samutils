use dioxus::prelude::*;

mod components;
mod tools;
use tools::QrCodeTool;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const MANIFEST: Asset = asset!("/assets/manifest.json");
const REGISTER_SW: Asset = asset!("/assets/register-sw.js");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/qr")]
    QrCode {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "manifest", href: MANIFEST }
        document::Meta { name: "theme-color", content: "#000000" }
        document::Script { src: REGISTER_SW }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut dark = use_signal(|| false);
    let theme = if *dark.read() { "bg-black text-white" } else { "bg-white text-black" };
    let border = if *dark.read() { "border-white" } else { "border-black" };
    let btn_cls = format!("border px-4 py-2 text-sm cursor-pointer transition-colors duration-300 hover:bg-[#888] hover:text-white hover:border-[#888] {border}");

    rsx! {
        div {
            class: "min-h-screen flex flex-col items-center px-2 py-4 transition-colors duration-300 {theme}",
            style: "font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;",

            header {
                class: "flex justify-between items-center w-full max-w-xl mb-8",
                h1 { class: "text-2xl font-normal tracking-widest", "samutil" }
                button {
                    class: "{btn_cls}",
                    onclick: move |_| { let v = *dark.read(); dark.set(!v); },
                    if *dark.read() { "LIGHT" } else { "DARK" }
                }
            }

            div {
                class: "w-full max-w-xl flex flex-col gap-2",

                Link {
                    to: Route::QrCode {},
                    class: "border p-4 text-left no-underline transition-colors duration-300 hover:bg-[#888] hover:text-white hover:border-[#888] {border} {theme}",
                    "QR CODE GENERATOR"
                }
            }
        }
    }
}

#[component]
fn QrCode() -> Element {
    rsx! { QrCodeTool {} }
}
