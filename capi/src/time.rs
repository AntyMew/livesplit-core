//! A time that can store a Real Time and a Game Time. Both of them are
//! optional.

use livesplit_core::{Time, TimingMethod};
use super::{acc, acc_mut, alloc, own_drop};
use std::ptr;
use time_span::{NullableTimeSpan};

/// type
pub type OwnedTime = *mut Time;

/// Creates a new Time with empty Real Time and Game Time.
#[no_mangle]
pub unsafe extern "C" fn Time_new() -> OwnedTime {
    alloc(Time::new())
}

/// Creates a new Time where Real Time and Game Time are zero. Keep in mind
/// that a zero Time Span is not the same as an empty Time Span.
#[no_mangle]
pub unsafe extern "C" fn Time_zero() -> OwnedTime {
    alloc(Time::zero())
}

/// Clones the time.
#[no_mangle]
pub unsafe extern "C" fn Time_clone(this: *const Time) -> OwnedTime {
    alloc(*acc(this))
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn Time_drop(this: OwnedTime) {
    own_drop(this);
}

/// The Real Time value. This may be <NULL> if this time has no Real Time value.
#[no_mangle]
pub unsafe extern "C" fn Time_real_time(this: *const Time) -> *const NullableTimeSpan {
    acc(this)
        .real_time
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}

/// The Game Time value. This may be <NULL> if this time has no Game Time value.
#[no_mangle]
pub unsafe extern "C" fn Time_game_time(this: *const Time) -> *const NullableTimeSpan {
    acc(this)
        .game_time
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}

/// Set the Real Time value.
#[no_mangle]
pub unsafe extern "C" fn Time_set_real_time(this: *mut Time, time: *const NullableTimeSpan) {
    acc_mut(this).real_time = if time == ptr::null() {
        None
    } else {
        Some(acc(time).clone())
    };
}

/// Set the Game Time value.
#[no_mangle]
pub unsafe extern "C" fn Time_set_game_time(this: *mut Time, time: *const NullableTimeSpan) {
    acc_mut(this).game_time = if time == ptr::null() {
        None
    } else {
        Some(acc(time).clone())
    };
}

/// Access the time's value for the timing method specified.
#[no_mangle]
pub unsafe extern "C" fn Time_index(
    this: *const Time,
    timing_method: TimingMethod,
) -> *const NullableTimeSpan {
    acc(this)[timing_method]
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}

/// Set the time's value for the timing method specified.
#[no_mangle]
pub unsafe extern "C" fn Time_set_index(
    this: *mut Time,
    timing_method: TimingMethod,
    time: *const NullableTimeSpan,
) {
    acc_mut(this)[timing_method] = if time == ptr::null() {
        None
    } else {
        Some(acc(time).clone())
    };
}

/// Create a new time from the result of adding this time to another time.
#[no_mangle]
pub unsafe extern "C" fn Time_add(this: *const Time, rhs: *const Time) -> OwnedTime {
    alloc(*acc(this) + *acc(rhs))
}

/// Create a new time from the result of subtracting another time from this
/// time.
#[no_mangle]
pub unsafe extern "C" fn Time_sub(this: *const Time, rhs: *const Time) -> OwnedTime {
    alloc(*acc(this) - *acc(rhs))
}

/// Add another time to this time.
#[no_mangle]
pub unsafe extern "C" fn Time_add_assign(this: *mut Time, rhs: *const Time) {
    *acc_mut(this) += *acc(rhs);
}

/// Subtracts another time from this time.
#[no_mangle]
pub unsafe extern "C" fn Time_sub_assign(this: *mut Time, rhs: *const Time) {
    *acc_mut(this) -= *acc(rhs);
}
