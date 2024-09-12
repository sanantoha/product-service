
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    MongoQuery(#[from] mongodb::error::Error),

    #[error("Failed to retrieve inserted ID {0}")]
    MongoKey(String),

    #[error("Can not parse variable: {input}")]
    Var {
        input: &'static str,
        #[source]
        source: std::env::VarError,
    },

    #[error(transparent)]
    AddParse(#[from] std::net::AddrParseError),
}