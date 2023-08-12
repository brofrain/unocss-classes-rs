# unocss-classes

> HTML class builder macro with UnoCSS variant group transformer for Rust web frameworks

This crate provides a wrapper around [classes!](https://crates.io/crates/classes) macro that additionally transforms string literals in harmony with [@unocss/transformer-variant-group](https://github.com/unocss/unocss/tree/main/packages/transformer-variant-group). It is meant to simplify building compound DOM classes in frontend Rust frameworks like [Leptos](https://leptos.dev/) / [Dioxus](https://dioxuslabs.com/) / [Yew](https://yew.rs/), while also taking full advantage of [UnoCSS](https://unocss.dev/)'s features.

## Exports

- `uno!` - works just like `classes!` macro, but also transforms Variant Groups in string literals. The transformation is done at compile time, so there is no overhead.
- `to_uno!` - it has the same purpose, but the output string is additionally transformed at **runtime**. It can be useful to transform dynamically combined classes, but generally should be avoided. Requires `runtime` feature.\
  Also UnoCSS may not be able to pick up utilities generated at runtime, so it may be necessary to use [Safelist](https://unocss.dev/guide/extracting#safelist) or [runtime engine](https://unocss.dev/integrations/runtime#runtime).

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

That's where the `to_uno!` macro comes in handy.

```rust
to_uno![Some("text-(sm center)")]

let class = "text-(sm center)";
to_uno![class]
```

### [Leptos](https://leptos.dev/) example

```rust
use leptos::*;
use unocss_classes::uno;

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

Dioxus on its own does not allow `String` to be used directly in `rsx!` as it has no `IntoAttributeValue` trait implemented. Some built-in componets even strictly require `&'a str` as the class attribute value.\
For that purpose, both `uno!` and `to_uno!` macros accept Dioxus' `Scope` as the first argument separated by a semicolon from the rest of the parameters. In such scenario, the output `String` will be converted to `&'a str` with a valid lifetime.

```rust
use dioxus::prelude::*;
use unocss_classes::uno;

pub fn App(cx: Scope) -> Element {
    render! {div { class: uno![cx; "hover:(bg-gray-400 font-medium)", "font-(light mono)"], "Some text" }}
}
```

There is an example Dioxus [app](https://github.com/brofrain/unocss-variant-group-transformer-rs/tree/main/examples/dioxus) as well!

### [Yew](https://yew.rs/) example

```rust
use yew::prelude::*;
use unocss_classes::uno;

#[function_component]
pub fn App() -> Html {
    html! { <div class={ uno!["hover:(bg-gray-400 font-medium)", "font-(light mono)"] }>{ "Some text" }</div> }
}
```

## Using `uno!` macro globally

```rust
#[macro_use]
extern crate unocss_classes;
```

## Using UnoCSS with a Rust front-end framework

You basically need to run [@unocss/cli](https://unocss.dev/integrations/cli#cli) first to build a CSS file embedded in your app and then run a bundler (e.g. Trunk) to compile the rest of your project. To do this automatically you can use [Cargo's build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) or [Trunk's pre-build hooks](https://github.com/thedodd/trunk/blob/master/Trunk.toml). For larger projects running in watch mode, it may be better to run `@unocss/cli` in watch mode as well, so building the output CSS file is a bit faster.

You can refer to the already mentioned [Leptos & Trunk example](https://github.com/brofrain/unocss-variant-group-transformer-rs/tree/main/examples/leptos) taking advantage of Trunk's hooks.

It is also possible to combine [Vite](https://vitejs.dev/) with Trunk / Dioxus CLI / Leptos CLI, but it requires a more hacky setup and, unless you really need some Vite-exclusive features (e.g. [Webfont self-hosting](https://github.com/feat-agency/vite-plugin-webfont-dl)), it's probably not worth the effort.

## License

[MIT License](https://opensource.org/licenses/MIT)

Copyright (c) 2023-PRESENT Kajetan Welc

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
