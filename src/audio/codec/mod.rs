#[cfg(feature = "seed")]
mod ak4556;
#[cfg(feature = "seed")]
pub use ak4556::{Codec, Pins};

#[cfg(feature = "seed_1_1")]
mod wm8731;
#[cfg(feature = "seed_1_1")]
pub use wm8731::{Codec, Pins};
