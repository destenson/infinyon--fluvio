mod error;
mod config;

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        mod core;
        mod services;
        mod start;
        mod controllers;
        mod smart_stream;

        pub use start::main_loop;
    }
}

use self::error::InternalServerError;
pub use config::SpuOpt;
