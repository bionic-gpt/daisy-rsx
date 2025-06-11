## Daisy-RSX

This is a [Dioxus](https://dioxuslabs.com/) version of the [Daisy UI](https://daisyui.com/) components.

## Installation

Install **daisy_rsx** using [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html):

```bash
cargo add daisy_rsx
```

This command will add the crate to your `Cargo.toml`:

```toml
[dependencies]
daisy_rsx = "0.1"
```

For the CSS part you can compile Tailwind and DaisyUI with [tailwind-cli-extra](https://github.com/dobicinaitis/tailwind-cli-extra) which does not require `npm`.
A typical `tailwind.css` might look like:

```css
@import 'tailwindcss';
@plugin "daisyui";

@source '../web-pages/**/*.rs';
@source 'typescript/**/*.ts';
@source inline("modal modal-box modal-action");
```

The `@source inline` directives ensure all Daisy UI classes used by Daisy RSX components are included in the final CSS.

Run `tailwindcss` (or `tailwind-cli-extra`) to produce your stylesheet and you are ready to use the components in your Dioxus application.

## Examples

Below are two simple examples showing how to use the components.

```rust
use dioxus::prelude::*;
use daisy_rsx::{Card, CardBody, CardHeader};

fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        Card {
            class: "w-64 bg-base-100 shadow-xl",
            CardBody { "A basic card" }
        }
    ))
}
```

With a button inside the card:

```rust
use dioxus::prelude::*;
use daisy_rsx::{Button, Card, CardBody, CardHeader};

fn App(cx: Scope) -> Element {
    cx.render(rsx!(
        Card {
            class: "w-64 bg-base-100 shadow-xl",
            CardHeader { title: "Actions" }
            CardBody {
                Button { class: "btn-primary", "Click me" }
            }
        }
    ))
}
```

## To Create a Release

To create a new release, use the following command locally:

```sh
cargo release patch
```

Once you are ready, pass the `--execute` flag.

This will:

- Bump the version number.
- Create a git tag.
- Push changes to the remote repository.
- Trigger the GitHub Actions workflow to publish the crate.
