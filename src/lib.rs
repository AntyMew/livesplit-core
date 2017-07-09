#![allow(unknown_lints)]

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate odds;
extern crate serde;
extern crate serde_json;
extern crate sxd_document;
#[macro_use]
extern crate quick_error;
extern crate base64;
extern crate byteorder;
extern crate image as imagelib;
#[macro_use]
extern crate derive_more;
extern crate unicase;
extern crate ntp;
#[macro_use]
extern crate lazy_static;

pub extern crate parking_lot;
pub extern crate livesplit_hotkey as hotkey;
pub extern crate ordermap;

mod atomic_clock_synchronization;
mod atomic_date_time;
mod attempt;
mod color;
mod hotkey_config;
mod hotkey_system;
mod image;
mod run_metadata;
mod run;
mod segment_history;
mod segment;
mod time_stamp;
mod time;
mod timer_phase;
mod timer;
mod timing_method;
pub mod analysis;
pub mod comparison;
pub mod component;
pub mod layout;
pub mod parser;
pub mod run_editor;
pub mod saver;
pub mod time_formatter;
pub mod time_span;

pub use chrono::{DateTime, Utc};
pub use self::atomic_date_time::AtomicDateTime;
pub use self::attempt::Attempt;
pub use self::color::Color;
pub use self::image::Image;
pub use self::run::Run;
pub use self::run_metadata::RunMetadata;
pub use self::segment_history::SegmentHistory;
pub use self::segment::Segment;
pub use self::run_editor::RunEditor;
pub use self::time::{Time, RealTime, GameTime};
pub use self::time_span::TimeSpan;
pub use self::time_stamp::TimeStamp;
pub use self::timer::{Timer, SharedTimer};
pub use self::timer_phase::TimerPhase;
pub use self::timing_method::TimingMethod;
pub use self::hotkey_config::HotkeyConfig;
pub use self::hotkey_system::HotkeySystem;
pub use self::layout::{Layout, Component, editor as layout_editor};
