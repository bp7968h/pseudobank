pub mod configuration;
pub mod clients;
pub mod service;
pub mod gateway;
pub mod registry;
pub mod router;

pub use configuration::{get_settings, Settings};
pub use service::handle_request;
pub use registry::Registry;