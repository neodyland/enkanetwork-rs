#[cfg(not(target_arch = "wasm32"))]
pub mod io_async;
#[cfg(target_arch = "wasm32")]
pub mod io_std;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use io_async::{read_file, write_file};
#[cfg(target_arch = "wasm32")]
pub(crate) use io_std::{read_file, write_file};

mod memory_cache;
pub use memory_cache::MemoryCache;
