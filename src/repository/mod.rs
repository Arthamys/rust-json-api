use diesel::pg::PgConnection;

// Create a database fairing so that rocket can manage our Database connection
// This marker signals what entry to use in `Rocket.toml`
// for more information, see: https://rocket.rs/v0.4/guide/state/#databases
#[database("pgdb")]
pub struct DbConn(PgConnection);

pub mod documents;
