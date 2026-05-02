use dioxus::prelude::*;

#[component]
pub fn ToolHeader(title: String, dark: bool, on_toggle_dark: EventHandler<()>) -> Element {
    let border_cls = if dark { "border-white" } else { "border-black" };
    let text_cls = if dark { "text-white" } else { "text-black" };

    rsx! {
        header {
            class: "grid grid-cols-3 items-center w-full max-w-xl mb-4",
            a {
                href: "/",
                class: "text-[#888] text-sm hover:text-current transition-colors justify-self-start",
                "← samutil"
            }
            span {
                class: "text-2xl font-normal tracking-widest justify-self-center",
                "{title}"
            }
            button {
                class: "border px-4 py-2 text-sm cursor-pointer transition-colors duration-300 hover:bg-[#888] hover:text-white hover:border-[#888] justify-self-end {border_cls} {text_cls}",
                onclick: move |_| on_toggle_dark.call(()),
                if dark { "LIGHT" } else { "DARK" }
            }
        }
    }
}
