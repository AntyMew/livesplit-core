use livesplit_core::component::text::Component as TextComponent;
use super::{Json, alloc, str, acc_mut, own_drop, acc, output_vec};
use text_component_state::OwnedTextComponentState;
use libc::c_char;

pub type OwnedTextComponent = *mut TextComponent;

#[no_mangle]
pub unsafe extern "C" fn TextComponent_new() -> OwnedTextComponent {
    alloc(TextComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn TextComponent_drop(this: OwnedTextComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TextComponent_state_as_json(this: *const TextComponent) -> Json {
    output_vec(|o| { acc(this).state().write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn TextComponent_set_center(this: *mut TextComponent, text: *const c_char) {
    acc_mut(this).settings_mut().text.set_center(str(text));
}

#[no_mangle]
pub unsafe extern "C" fn TextComponent_set_left(this: *mut TextComponent, text: *const c_char) {
    acc_mut(this).settings_mut().text.set_left(str(text));
}

#[no_mangle]
pub unsafe extern "C" fn TextComponent_set_right(this: *mut TextComponent, text: *const c_char) {
    acc_mut(this).settings_mut().text.set_right(str(text));
}

#[no_mangle]
pub unsafe extern "C" fn TextComponent_state(
    this: *const TextComponent,
) -> OwnedTextComponentState {
    alloc(acc(this).state())
}
