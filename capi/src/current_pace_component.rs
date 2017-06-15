use livesplit_core::component::current_pace::Component as CurrentPaceComponent;
use livesplit_core::Timer;
use super::{Json, alloc, own_drop, acc, output_vec, acc_mut};
use current_pace_component_state::OwnedCurrentPaceComponentState;

pub type OwnedCurrentPaceComponent = *mut CurrentPaceComponent;

#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponent_new() -> OwnedCurrentPaceComponent {
    alloc(CurrentPaceComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponent_drop(this: OwnedCurrentPaceComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponent_state_as_json(
    this: *mut CurrentPaceComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponent_state(
    this: *mut CurrentPaceComponent,
    timer: *const Timer,
) -> OwnedCurrentPaceComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}
