use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {

    view! {

        <main>

            <section>

                <div class="section__heading">
                    <h1>Welcome to Lydcuit!</h1>
                    <p>A mobile-first Discuit client, built with Tauri and Leptos.</p>
                </div>

                <div class="section__content">

                    <p>Lydcuit is a mobile-first Discuit client, built with Tauri and Leptos.
                        It is designed to be a simple, fast, and easy-to-use client for Discuit,
                        with a focus on mobile devices.
                    </p>

                    <p>Lydcuit is built with Tauri, a toolkit for building desktop apps with
                        web technologies. It is designed to be a simple, fast, and
                        easy-to-use client for Discuit, with a focus on mobile devices.
                    </p>

                    <div class="link__row">

                        <a href="https://leptos.dev/" target="_blank" rel="noopener noreferrer">
                            <img src="https://leptos.dev/images/header_logo.svg" alt="Leptos" />
                        </a>

                        <a href="https://tauri.studio/" target="_blank" rel="noopener noreferrer">
                            <img src="https://tauri.app/meta/tauri_logo_dark.svg" alt="Tauri" />
                        </a>

                        <a href="https://rust-lang.org/" target="_blank" rel="noopener noreferrer">
                            <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png" alt="Rust" />
                        </a>

                    </div>

                </div>


            </section>

        </main>

    }

}