// TODO: These should be guarded with a feature flag or similar. Currently only terminal is implemented, so it does not really matter.
pub mod terminal;
pub use terminal as renderer;