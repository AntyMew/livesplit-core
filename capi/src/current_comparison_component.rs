use livesplit_core::component::current_comparison::Component as CurrentComparisonComponent;
use livesplit_core::Timer;
use super::{Json, alloc, own_drop, acc, output_vec, acc_mut};
use current_comparison_component_state::OwnedCurrentComparisonComponentState;

pub type OwnedCurrentComparisonComponent = *mut CurrentComparisonComponent;

#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponent_new() -> OwnedCurrentComparisonComponent {
    alloc(CurrentComparisonComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponent_drop(this: OwnedCurrentComparisonComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponent_state_as_json(
    this: *mut CurrentComparisonComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponent_state(
    this: *mut CurrentComparisonComponent,
    timer: *const Timer,
) -> OwnedCurrentComparisonComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}
