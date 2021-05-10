fn main() {
    if cfg!(target_os = "windows") {
        windows::build!(
            Windows::Win32::WindowsAndMessaging::{SystemParametersInfoW, SYSTEM_PARAMETERS_INFO_ACTION, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS},
        );
    }
}
