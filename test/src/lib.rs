use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use enkanetwork_rs::{CharacterId, EnkaNetwork, IconData};
use once_cell::sync::OnceCell;
use std::str::FromStr;
use itertools::join;
use base64::{Engine as _, engine::general_purpose};

static ENKA: OnceCell<EnkaNetwork> = OnceCell::new();

static ICON_DATA: OnceCell<IconData> = OnceCell::new();

/// Load
#[wasm_bindgen]
pub async fn w_load() -> Result<(),String> {
    let enka = EnkaNetwork::new().map_err(|e| e.to_string())?;
    let icon_data = enka.icon_data().await;
    let e = ENKA.set(enka);
    if e.is_err() {
        return Err("EnkaNetwork already loaded".to_string());
    }
    let e = ICON_DATA.set(icon_data);
    if e.is_err() {
        return Err("IconData already loaded".to_string());
    }
    Ok(())
}
