use livesplit_core::component::total_playtime::Component as TotalPlaytimeComponent;
use livesplit_core::Timer;
use super::{Json, alloc, own_drop, acc, output_vec, acc_mut};
use total_playtime_component_state::OwnedTotalPlaytimeComponentState;

pub type OwnedTotalPlaytimeComponent = *mut TotalPlaytimeComponent;

#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponent_new() -> OwnedTotalPlaytimeComponent {
    alloc(TotalPlaytimeComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponent_drop(this: OwnedTotalPlaytimeComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponent_state_as_json(
    this: *mut TotalPlaytimeComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponent_state(
    this: *mut TotalPlaytimeComponent,
    timer: *const Timer,
) -> OwnedTotalPlaytimeComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}
