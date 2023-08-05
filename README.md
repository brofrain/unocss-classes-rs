# unocss-variant-group-transformer

> UnoCSS variant group transformer macro for Rust web frameworks

This crate provides `uno!` macro that applies [@unocss/transformer-variant-group](https://github.com/unocss/unocss/tree/main/packages/transformer-variant-group) to string literals.

## [Leptos](https://leptos.dev/) example

```rust
use leptos::*;
use unocss_variant_group_transformer::uno;

#[component]
fn App() -> impl IntoView {
    view! { <div class=uno!("hover:(bg-gray-400 font-medium)", "font-(light mono)")>"Some text"</div> }
    // equivalent to: <div class="hover:bg-gray-400 hover:font-medium font-light font-mono">Some text</div>
}
```

Check out also this example [app](https://github.com/brofrain/unocss-variant-group-transformer-rs/tree/main/examples/leptos)

## [Dioxus](https://dioxuslabs.com/) example

```rust
use dioxus::prelude::*;
use unocss_variant_group_transformer::uno;

pub fn App(cx: Scope) -> Element {
    render! {div { class: uno!("hover:(bg-gray-400 font-medium)", "font-(light mono)"), "Some text" }}
}
```

## [Yew](https://yew.rs/) example

```rust
use yew::prelude::*;
use unocss_variant_group_transformer::uno;

#[function_component]
pub fn App() -> Html {
    html! { <div class={ uno!("hover:(bg-gray-400 font-medium)", "font-(light mono)") }>{ "Some text" }</div> }
}
```

## Using `uno!` macro globally

```rust
#[macro_use]
extern crate yew_unocss_transformer;
```

## Using UnoCSS with a Rust front-end framework

// TODO

## License

[MIT License](https://opensource.org/licenses/MIT)

Copyright (c) 2023-PRESENT Kajetan Welc

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
