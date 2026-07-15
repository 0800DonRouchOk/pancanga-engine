//! Calendar Engine.
//!
//! This module contains civil-time calendar calculations built on top of the
//! astronomical engine. It does not contain Vaishnava rules.

mod sunrise;
mod tithi;

pub use sunrise::sunrise;
pub use tithi::{
    tithi_at_sunrise, tithi_presence_between_sunrises, tithi_transition_between_sunrises,
    CivilDayTithiPresence, TithiTransition,
};
