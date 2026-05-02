use crate::components::ToolHeader;
use dioxus::prelude::*;
use qrcodegen::{QrCode, QrCodeEcc};

fn qr_to_svg(qr: &QrCode, dark_mode: bool) -> String {
    let border = 1i32;
    let size = qr.size();
    let dim = size + border * 2;
    let mut paths = String::new();
    for y in 0..size {
        for x in 0..size {
            if qr.get_module(x, y) {
                paths.push_str(&format!("M{},{}h1v1h-1z ", x + border, y + border));
            }
        }
    }
    let (bg, fg) = if dark_mode { ("#000000", "#ffffff") } else { ("#ffffff", "#000000") };
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {dim} {dim}" shape-rendering="crispEdges" style="width:100%;height:100%"><rect width="100%" height="100%" fill="{bg}"/><path d="{paths}" fill="{fg}"/></svg>"#,
        dim = dim,
        paths = paths.trim(),
        bg = bg,
        fg = fg,
    )
}

#[component]
pub fn QrCodeTool() -> Element {
    let mut text = use_signal(|| String::new());
    let mut dark = use_signal(|| false);

    let svg = use_memo(move || {
        let t = text.read().trim().to_string();
        if t.is_empty() {
            return None;
        }
        QrCode::encode_text(&t, QrCodeEcc::Medium)
            .ok()
            .map(|qr| qr_to_svg(&qr, *dark.read()))
    });

    let has_qr = svg.read().is_some();

    let theme = if *dark.read() { "bg-black text-white" } else { "bg-white text-black" };
    let border_cls = if *dark.read() { "border-white" } else { "border-black" };
    let text_cls = if *dark.read() { "text-white" } else { "text-black" };
    let btn_base = format!("p-4 text-base border cursor-pointer transition-colors duration-300 hover:bg-[#888] hover:text-white hover:border-[#888] {border_cls} {text_cls}");
    let btn_disabled = format!("{btn_base} opacity-30 cursor-not-allowed");
    let input_cls = format!(
        "w-full p-4 border text-base resize-y min-h-28 outline-none focus:border-[#888] placeholder-[#888] transition-colors duration-300 {} {}",
        if *dark.read() { "bg-black text-white border-white" } else { "bg-white text-black border-black" },
        ""
    );

    rsx! {
        div {
            class: "min-h-screen flex flex-col items-center px-2 py-4 transition-colors duration-300 {theme}",
            style: "font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;",

            ToolHeader {
                title: "qr code",
                dark: *dark.read(),
                on_toggle_dark: move |_| { let v = *dark.read(); dark.set(!v); },
            }

            div {
                class: "w-full max-w-xl flex flex-col gap-4 flex-1",

                

                // Controls
                div {
                    class: "flex flex-col gap-4",

                    textarea {
                        class: "{input_cls}",
                        placeholder: "Enter text or URL to encode...",
                        autocomplete: "off",
                        spellcheck: false,
                        oninput: move |e| text.set(e.value()),
                        value: "{text}",
                    }

                    div {
                        class: "grid grid-cols-2 gap-4",

                        button {
                            class: if has_qr { "{btn_base}" } else { "{btn_disabled}" },
                            disabled: !has_qr,
                            onclick: {
                                let svg_sig = svg.clone();
                                let text_sig = text.clone();
                                move |_| {
                                    if let Some(s) = svg_sig.read().clone() {
                                        let t = text_sig.read().clone();
                                        spawn(async move { share_qr(s, t).await; });
                                    }
                                }
                            },
                            "SHARE"
                        }

                        button {
                            class: if has_qr { "{btn_base}" } else { "{btn_disabled}" },
                            disabled: !has_qr,
                            onclick: {
                                let svg_sig = svg.clone();
                                let text_sig = text.clone();
                                move |_| {
                                    if let Some(s) = svg_sig.read().clone() {
                                        let t = text_sig.read().clone();
                                        spawn(async move { download_qr(s, t).await; });
                                    }
                                }
                            },
                            "DOWNLOAD"
                        }
                    }
                }

                // QR display
                div {
                    class: "flex-1 flex flex-col items-center justify-center border min-h-80 p-4 {border_cls}",
                    if let Some(svg_str) = svg.read().as_ref() {
                        div {
                            class: "w-72 h-72",
                            dangerous_inner_html: "{svg_str}",
                        }
                    } else {
                        span {
                            class: "text-[#888] text-sm text-center",
                            "Enter text below to generate QR code"
                        }
                    }
                }
            }

            

            a {
                href: "/",
                class: "mt-8 text-[#888] text-sm hover:text-current transition-colors",
                "← BACK TO HOME"
            }
        }
    }
}

async fn share_qr(svg: String, text: String) {
    let filename = sanitize_filename(&text);
    let js = format!(
        r#"(async () => {{
            const svg = {svg_json};
            const blob = new Blob([svg], {{type: 'image/svg+xml'}});
            const file = new File([blob], '{filename}.svg', {{type: 'image/svg+xml'}});
            if (navigator.share && navigator.canShare && navigator.canShare({{files: [file]}})) {{
                try {{ await navigator.share({{files: [file], title: 'QR Code'}}); return; }} catch(e) {{ if (e.name === 'AbortError') return; }}
            }}
            try {{ await navigator.clipboard.writeText({text_json}); alert('Copied to clipboard'); }} catch(e) {{}}
        }})();"#,
        svg_json = js_string(&svg),
        filename = filename,
        text_json = js_string(&text),
    );
    let _ = document::eval(&js).await;
}

async fn download_qr(svg: String, text: String) {
    let filename = sanitize_filename(&text);
    let js = format!(
        r#"(() => {{
            const blob = new Blob([{svg_json}], {{type: 'image/svg+xml'}});
            const url = URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url; a.download = '{filename}.svg';
            document.body.appendChild(a); a.click();
            document.body.removeChild(a);
            URL.revokeObjectURL(url);
        }})();"#,
        svg_json = js_string(&svg),
        filename = filename,
    );
    let _ = document::eval(&js).await;
}

fn js_string(s: &str) -> String {
    let escaped = s.replace('\\', "\\\\").replace('`', "\\`").replace('$', "\\$");
    format!("`{}`", escaped)
}

fn sanitize_filename(text: &str) -> String {
    let s: String = text
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    let s = s.trim_matches('-').to_string();
    if s.is_empty() { "qr-code".to_string() } else { s }
}
