//! Defines the arguments required to start the server application using [`clap`].
//!
//! [`clap`]: https://github.com/clap-rs/clap/

/// The configuration parameters for the application.
#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,
}
