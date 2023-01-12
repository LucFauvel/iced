//! Build window-based GUI applications.
mod action;
mod event;
mod mode;
mod redraw_request;
mod user_attention;

pub use action::Action;
pub use event::Event;
pub use mode::Mode;
pub use redraw_request::RedrawRequest;
pub use user_attention::UserAttention;

use crate::subscription::{self, Subscription};
use crate::time::Instant;

/// Subscribes to the frames of the window of the running application.
///
/// The resulting [`Subscription`] will produce items at a rate equal to the
/// framerate of the monitor of said window.
pub fn frames() -> Subscription<Instant> {
    subscription::raw_events(|event, _status| match event {
        crate::Event::Window(Event::RedrawRequested(at)) => Some(at),
        _ => None,
    })
}
