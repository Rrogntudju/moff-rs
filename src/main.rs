mod bindings {
    windows::include_bindings!();
}

use std::{mem::forget, usize};
use bindings::Windows::{
    Win32::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
    Win32::DisplayDevices::{RECT,},
    Win32::Monitor::{
            GetNumberOfPhysicalMonitorsFromHMONITOR, 
            GetPhysicalMonitorsFromHMONITOR, 
            DestroyPhysicalMonitor, 
            SetVCPFeature, 
            PHYSICAL_MONITOR },
    Win32::SystemServices::{HANDLE, BOOL,},
    Win32::WindowsAndMessaging::LPARAM,
};


unsafe extern "system" fn switch_proc(hmonitor: HMONITOR, _hdc: HDC, _rect: *mut RECT, _lparam: LPARAM) -> BOOL {
    let mon_count: u32 = 0;
    if  GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, mon_count as *mut u32) != 0 {
        if mon_count > 0 {
            let mut mons = Vec::<PHYSICAL_MONITOR>::with_capacity(mon_count as usize);
            let mons_ptr = mons.as_mut_ptr();
            forget(mons);
            if GetPhysicalMonitorsFromHMONITOR(hmonitor, mon_count, mons_ptr) != 0 {
                let mons = Vec::<PHYSICAL_MONITOR>::from_raw_parts(mons_ptr, mon_count as usize, mon_count as usize);
                for mon in mons {
                    if SetVCPFeature(mon.hPhysicalMonitor, 0xd6, ) == 0 {
                        print_last_error("SetVCPFeature");
                    }
                    if DestroyPhysicalMonitor(mon.hPhysicalMonitor) == 0 {
                        print_last_error("DestroyPhysicalMonitor");
                    }
                }
 
            } else {
                print_last_error("GetPhysicalMonitorsFromHMONITOR");
            }
        }
    } else {
        print_last_error("GetNumberOfPhysicalMonitorsFromHMONITOR");
    }

    BOOL(1)
}

fn print_last_error(err_func: &str) {
    match std::io::Error::last_os_error().raw_os_error() {
        Some(e) => eprintln!("{} a retourné le code d'erreur {}", err_func, e),
        None => eprintln!("DOH!"),
    }
}

fn main()  {
    if unsafe { !EnumDisplayMonitors(HDC::NULL, 0 as *mut RECT, Some(switch_proc), LPARAM::NULL).as_bool() } {
        print_last_error("EnumDisplayMonitors");
    }
}
