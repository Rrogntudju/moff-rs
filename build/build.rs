fn main() {
    windows::build!(
        Windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
        Windows::Win32::Foundation::{RECT, BOOL, HANDLE, PSTR, LPARAM},
        Windows::Win32::Devices::Display::{
            CapabilitiesRequestAndCapabilitiesReply,
            GetCapabilitiesStringLength,
            GetVCPFeatureAndVCPFeatureReply,
            GetNumberOfPhysicalMonitorsFromHMONITOR,
            GetPhysicalMonitorsFromHMONITOR,
            DestroyPhysicalMonitor,
            SetVCPFeature,
            PHYSICAL_MONITOR,
            MC_VCP_CODE_TYPE},
    );
}
