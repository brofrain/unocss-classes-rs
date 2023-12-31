use leptos::*;
use leptos_use::{use_color_mode_with_options, ColorMode, UseColorModeOptions, UseColorModeReturn};
use unocss_classes::{to_uno, uno};

#[component]
fn App() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default().transition_enabled(true));

    let toggle_dark = move |_| {
        set_mode(if mode() == ColorMode::Dark {
            ColorMode::Light
        } else {
            ColorMode::Dark
        })
    };

    let gh_icon = "icon-carbon-logo-github";

    view! {
        <div class=uno![
            "font-sans", "min-h-screen", "flex-(~ col) items-center gap6", "py10",
            "bg-#f5f5f5 text-#1a1a1a dark:(bg-#1a1a1a text-#fff) transition"
        ]>
            <div class="icon-carbon-sun dark:icon-carbon-moon text-8xl"></div>

            <div class=move || {
                uno!["font-serif", "italic" => mode() == ColorMode::Light]
            }>"Leptos + UnoCSS example"</div>

            <button
                class=uno![
                    "block", "rounded", "p-(x4 y1)", "fw600", "bg-#d74f3f/90 hover:bg-#f74c00",
                    "select-none", "cursor-pointer", "transition-background-color"
                ]

                on:click=toggle_dark
            >
                "Toggle dark mode"
            </button>

            <a
                class=to_uno![gh_icon, "text-3xl", Some("hover:text-#f74c00"), "transition"]
                href="https://github.com/brofrain/unocss-classes-rs"
                target="_blank"
            ></a>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
