fn main() {
    if cfg!(target_os = "windows") {
        windows::build!(
            Windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
            Windows::Win32::UI::DisplayDevices::{RECT,},
            Windows::Win32::Devices::Display::{
                CapabilitiesRequestAndCapabilitiesReply,
                GetCapabilitiesStringLength,
                GetVCPFeatureAndVCPFeatureReply,
                GetNumberOfPhysicalMonitorsFromHMONITOR,
                GetPhysicalMonitorsFromHMONITOR,
                DestroyPhysicalMonitor,
                SetVCPFeature,
                PHYSICAL_MONITOR,
                MC_VCP_CODE_TYPE,
                MC_SET_PARAMETER},
            Windows::Win32::System::SystemServices::{BOOL, HANDLE, PSTR},
            Windows::Win32::UI::WindowsAndMessaging::LPARAM,
        );
    }
}
