mod bindings {
    windows::include_bindings!();
}

use bindings::Windows::{
    Win32::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
    Win32::DisplayDevices::{RECT,},
    Win32::Monitor::{
            GetNumberOfPhysicalMonitorsFromHMONITOR, 
            GetPhysicalMonitorsFromHMONITOR, 
            DestroyPhysicalMonitor, 
            SetVCPFeature, 
            PHYSICAL_MONITOR },
    Win32::SystemServices::{HANDLE, BOOL},
    Win32::WindowsAndMessaging::LPARAM,
};
use std::error::Error;

unsafe extern "system" fn turnOffProc(hmonitor: HMONITOR, hdc: HDC, rect: *mut RECT, lparam: LPARAM) -> BOOL {
    BOOL(1)
}

fn main() -> Result<(), Box<dyn Error>> {

    let rc = unsafe { EnumDisplayMonitors(HDC::NULL, 0 as *mut RECT, Some(turnOffProc), LPARAM::NULL) };
    if !rc.as_bool() {
        return Err(match std::io::Error::last_os_error().raw_os_error() {
            Some(e) => format!("EnumDisplayMonitors a retourné le code d'erreur {}", e).into(),
            None => "Oups!".into(),
        })
    }

    Ok(())
}
