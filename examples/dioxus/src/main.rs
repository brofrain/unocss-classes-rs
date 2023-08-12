#![allow(non_snake_case)]
use dioxus::prelude::*;
use gloo::utils::window;
use unocss_classes::{to_uno, uno};

fn html_el() -> web_sys::Element {
    window()
        .document()
        .unwrap()
        .query_selector("html")
        .unwrap()
        .unwrap()
}

fn App(cx: Scope) -> Element {
    let dark = use_state(cx, || false);

    let toggle_dark = |_| {
        if *dark.get() {
            html_el().set_class_name("");
            dark.set(false);
        } else {
            html_el().set_class_name("dark");
            dark.set(true);
        }
    };

    let gh_icon = "icon-carbon-logo-github";

    render! {
        div { class: uno![
                cx; "font-sans", "min-h-screen", "flex-(~ col) items-center gap6", "py10",
                "bg-#f5f5f5 text-#1a1a1a dark:(bg-#1a1a1a text-#fff) transition"
            ],
            div { class: "icon-carbon-sun dark:icon-carbon-moon text-8xl" }

            div { class: uno![cx; "font-serif", "italic" => !* dark.get()], "Dioxus + UnoCSS example" }

            button { class: uno![
                    cx; "block", "rounded", "p-(x4 y1)", "fw600", "bg-#d74f3f/90 hover:bg-#f74c00",
                    "select-none", "cursor-pointer", "transition-background-color"
                ], onclick: toggle_dark, "Toggle dark mode" }

            a {
                class: to_uno![cx; gh_icon, "text-3xl", "hover:text-#f74c00", "transition"],
                href: "https://github.com/brofrain/unocss-classes-rs",
                target: "_blank"
            }
        }
    }
}

fn main() {
    dioxus_web::launch(App);
}
