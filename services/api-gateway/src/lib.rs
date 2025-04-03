pub mod configuration;
pub mod clients;
pub mod service;

pub use configuration::get_settings;
pub use service::handle_request;