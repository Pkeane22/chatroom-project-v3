pub mod api;
pub mod app;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      _ = console_log::init_with_level(log::Level::Debug);
      console_error_panic_hook::set_once();

      leptos::mount_to_body(App);
    }
}
}
cfg_if! {
if #[cfg(feature = "ssr")] {
    use leptos::LeptosOptions;
    use sqlx::PgPool;

    pub struct AppData {
        leptos_options: LeptosOptions,
        pool: PgPool,
    }

    impl AppData {
        pub fn new(leptos_options: LeptosOptions, pool: PgPool) -> Self {
            AppData {
                leptos_options,
                pool,
            }
        }
}
}
}
