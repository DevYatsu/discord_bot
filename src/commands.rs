use crate::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod decret;
pub use decret::decret;
mod goulag;
pub use goulag::goulag;
mod liberation;
pub use liberation::liberation;
mod hymn;
pub use hymn::hymn;
mod activite;
pub use activite::activite;