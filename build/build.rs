fn main() {
    if cfg!(target_os = "windows") {
        windows::build!(
            Windows::Win32::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
            Windows::Win32::DisplayDevices::{RECT,},
            Windows::Win32::Monitor::{
                CapabilitiesRequestAndCapabilitiesReply,
                GetCapabilitiesStringLength,
                GetNumberOfPhysicalMonitorsFromHMONITOR,
                GetPhysicalMonitorsFromHMONITOR,
                DestroyPhysicalMonitor,
                SetVCPFeature,
                GetVCPFeatureAndVCPFeatureReply,
                PHYSICAL_MONITOR,
                MC_VCP_CODE_TYPE },
            Windows::Win32::SystemServices::{BOOL, HANDLE, PSTR},
            Windows::Win32::WindowsAndMessaging::LPARAM,
        );
    }
}
