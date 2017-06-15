use livesplit_core::component::sum_of_best::Component as SumOfBestComponent;
use livesplit_core::Timer;
use super::{Json, alloc, own_drop, acc, output_vec};
use sum_of_best_component_state::OwnedSumOfBestComponentState;

pub type OwnedSumOfBestComponent = *mut SumOfBestComponent;

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_new() -> OwnedSumOfBestComponent {
    alloc(SumOfBestComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_drop(this: OwnedSumOfBestComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_state_as_json(
    this: *const SumOfBestComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_state(
    this: *const SumOfBestComponent,
    timer: *const Timer,
) -> OwnedSumOfBestComponentState {
    alloc(acc(this).state(acc(timer)))
}
