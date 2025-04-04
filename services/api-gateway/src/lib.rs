pub mod configuration;
pub mod clients;
pub mod service;
pub mod gateway;

pub use configuration::get_settings;
pub use service::handle_request;