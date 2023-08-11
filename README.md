# unocss-variant-group-transformer

> HTML class builder macro with UnoCSS variant group transformer for Rust web frameworks

This crate provides a wrapper around [classes!](https://crates.io/crates/classes) macro that additionally transforms string literals in harmony with [@unocss/transformer-variant-group](https://github.com/unocss/unocss/tree/main/packages/transformer-variant-group). It is meant to simplify building compound DOM classes in frontend Rust frameworks like [Leptos](https://leptos.dev/) / [Dioxus](https://dioxuslabs.com/) / [Yew](https://yew.rs/), while also taking full advantage of [UnoCSS](https://unocss.dev/)'s features.

## Exports

- `uno!` - works just like `classes!` macro, but also transforms Variant Groups in string literals. The transformation is done at compile time, so there is no overhead.
- `to_uno!` - it has the same purpose, but the transformation is executed at **runtime**. It can be useful to transform dynamically combined classes, but generally should be avoided. Requires `runtime` feature.

## Examples

- ```rust
  uno!["hover:(bg-gray-400 font-medium)", "font-(light mono)"]; // -> "hover:bg-gray-400 hover:font-medium font-light font-mono"
  ```

- ```rust
  let some = Some("text-red");
  let none = None::<String>;
  let truthy = true;
  let falsy = false;
  uno![
    some,
    none,
    "text-green" => truthy,
    "text-black" => falsy,
    Some("text-white").map(|_| "text-blue")
  ]; // -> "text-red text-green text-blue"
  ```

- ```rust
  let truthy = true;
  uno!["text-(sm center)" => truthy]; // -> "text-sm text-center"
  ```

Note that only pure string literals inside the macro's scope are transformed.

```rust
uno![Some("text-(sm center)")] // doesn't work

let class = "text-(sm center)";
uno![class] // doesn't work
```

### [Leptos](https://leptos.dev/) example

```rust
use leptos::*;
use unocss_variant_group_transformer::uno;

#[component]
fn App() -> impl IntoView {
    view! { <div class=uno!["hover:(bg-gray-400 font-medium)", "font-(light mono)"]>"Some text"</div> }
    // equivalent to: <div class="hover:bg-gray-400 hover:font-medium font-light font-mono">Some text</div>
}

// or as a reactive derived signal:
#[component]
fn App() -> impl IntoView {
    let reactive_condition = create_rw_signal(true);
    view! {
        <div class=move || uno!["hover:(bg-gray-400 font-medium)", "font-(light mono)" => reactive_condition()]>
            "Some text"
        </div>
    }
}
```

You can check out also this example Leptos [app](https://github.com/brofrain/unocss-variant-group-transformer-rs/tree/main/examples/leptos).

### [Dioxus](https://dioxuslabs.com/) example

```rust
use dioxus::prelude::*;
use unocss_variant_group_transformer::uno;

pub fn App(cx: Scope) -> Element {
    render! {div { class: uno!["hover:(bg-gray-400 font-medium)", "font-(light mono)"], "Some text" }}
}
```

### [Yew](https://yew.rs/) example

```rust
use yew::prelude::*;
use unocss_variant_group_transformer::uno;

#[function_component]
pub fn App() -> Html {
    html! { <div class={ uno!["hover:(bg-gray-400 font-medium)", "font-(light mono)"] }>{ "Some text" }</div> }
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
