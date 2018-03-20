//! A Time Span represents a certain span of time.

use livesplit_core::TimeSpan;
use super::{acc, acc_mut, alloc, own_drop, str};
use std::os::raw::c_char;
use std::ptr;
use std::str::FromStr;

/// type
pub type NullableTimeSpan = TimeSpan;
/// type
pub type OwnedTimeSpan = *mut TimeSpan;

/// Clones the Time Span.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_clone(this: *const TimeSpan) -> OwnedTimeSpan {
    alloc(*acc(this))
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_drop(this: OwnedTimeSpan) {
    own_drop(this);
}

/// Creates a new Time Span of zero length.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_zero() -> OwnedTimeSpan {
    alloc(TimeSpan::zero())
}

/// Creates a new Time Span from a given amount of milliseconds.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_from_milliseconds(milliseconds: f64) -> OwnedTimeSpan {
    alloc(TimeSpan::from_milliseconds(milliseconds))
}

/// Creates a new Time Span from a given amount of seconds.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_from_seconds(seconds: f64) -> OwnedTimeSpan {
    alloc(TimeSpan::from_seconds(seconds))
}

/// Creates a new Time Span from a given amount of days.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_from_days(days: f64) -> OwnedTimeSpan {
    alloc(TimeSpan::from_days(days))
}

/// Returns the total amount of seconds (including decimals) this Time Span
/// represents.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_total_seconds(this: *const TimeSpan) -> f64 {
    acc(this).total_seconds()
}

/// Returns the total amount of milliseconds (including decimals) this Time
/// Span represents.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_total_milliseconds(this: *const TimeSpan) -> f64 {
    acc(this).total_milliseconds()
}

/// Creates a new Time Span from a string. Returns <NULL> if the string could
/// not be parsed.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_from_str(text: *const c_char) -> OwnedTimeSpan {
    if let Ok(time_span) = TimeSpan::from_str(str(text)) {
        alloc(time_span)
    } else {
        ptr::null_mut()
    }
}

/// Creates a default Time Span.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_default() -> OwnedTimeSpan {
    alloc(TimeSpan::default())
}

/// Creates a new Time Span from the negation of this Time Span.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_neg(this: *const TimeSpan) -> OwnedTimeSpan {
    alloc(-*acc(this))
}

/// Creates a new Time Span from the result of another Time Span subtracted
/// from this Time Span.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_add(
    this: *const TimeSpan,
    rhs: *const TimeSpan,
) -> OwnedTimeSpan {
    alloc(*acc(this) + *acc(rhs))
}

/// Creates a new Time Span from the result of this Time Span added to another
/// Time Span.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_sub(
    this: *const TimeSpan,
    rhs: *const TimeSpan,
) -> OwnedTimeSpan {
    alloc(*acc(this) - *acc(rhs))
}

/// Creates a new Time Span from the result of this Time Span multiplied by an
/// integer.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_mul(this: *const TimeSpan, rhs: i32) -> OwnedTimeSpan {
    alloc(*acc(this) * rhs)
}

/// Creates a new Time Span from the result of this Time Span divided by an
/// integer.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_div(this: *const TimeSpan, rhs: i32) -> OwnedTimeSpan {
    alloc(*acc(this) / rhs)
}

/// Add another Time Span to this mutable Time Span.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_add_assign(this: *mut TimeSpan, rhs: *const TimeSpan) {
    *acc_mut(this) += *acc(rhs)
}

/// Subtract another Time Span from this mutable Time Span.
#[no_mangle]
pub unsafe extern "C" fn TimeSpan_sub_assign(this: *mut TimeSpan, rhs: *const TimeSpan) {
    *acc_mut(this) -= *acc(rhs)
}
