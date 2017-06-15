use livesplit_core::component::splits::State as SplitsComponentState;
use super::{own_drop, acc, output_str, output_str_with};
use libc::c_char;
use std::ptr;
use std::fmt::Write;

pub type OwnedSplitsComponentState = *mut SplitsComponentState;

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_drop(this: OwnedSplitsComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_final_separator_shown(
    this: *const SplitsComponentState,
) -> bool {
    acc(this).show_final_separator
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_len(this: *const SplitsComponentState) -> usize {
    acc(this).splits.len()
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_icon_change(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    acc(this).splits[index].icon_change.as_ref().map_or_else(
        ptr::null,
        |s| {
            output_str(s)
        },
    )
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_name(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&acc(this).splits[index].name)
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_delta(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&acc(this).splits[index].delta)
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_time(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str(&acc(this).splits[index].time)
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_color(
    this: *const SplitsComponentState,
    index: usize,
) -> *const c_char {
    output_str_with(|f| {
        write!(f, "{:?}", acc(this).splits[index].color).unwrap()
    })
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponentState_is_current_split(
    this: *const SplitsComponentState,
    index: usize,
) -> bool {
    acc(this).splits[index].is_current_split
}
