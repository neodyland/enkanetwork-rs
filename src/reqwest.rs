#[cfg(target_arch = "wasm32")]
pub use reqwest_wasm::*;
#[cfg(not(target_arch = "wasm32"))]
pub use reqwest::*;