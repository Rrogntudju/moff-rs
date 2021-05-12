mod bindings {
    windows::include_bindings!();
}

use bindings::Windows::{
    Win32::DisplayDevices::RECT,
    Win32::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
    Win32::Monitor::{
        DestroyPhysicalMonitor, GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR, GetVCPFeatureAndVCPFeatureReply,
        SetVCPFeature, MC_VCP_CODE_TYPE, PHYSICAL_MONITOR,
    },
    Win32::SystemServices::BOOL,
    Win32::WindowsAndMessaging::LPARAM,
};
use std::{mem::forget, usize};

unsafe extern "system" fn switch_proc(hmonitor: HMONITOR, _hdc: HDC, _rect: *mut RECT, _lparam: LPARAM) -> BOOL {
    let mut mon_count: u32 = 0;

    if GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut mon_count as *mut u32) != 0 {
        if mon_count > 0 {
            let mut mons = Vec::<PHYSICAL_MONITOR>::with_capacity(mon_count as usize);
            let mons_ptr = mons.as_mut_ptr();
            forget(mons);

            if GetPhysicalMonitorsFromHMONITOR(hmonitor, mon_count, mons_ptr) != 0 {
                let mons = Vec::<PHYSICAL_MONITOR>::from_raw_parts(mons_ptr, mon_count as usize, mon_count as usize);
                for mon in mons {
                    let mut current: u32 = 0;
                    let mut max: u32 = 0;
                    let mut vct = MC_VCP_CODE_TYPE::MC_SET_PARAMETER;

                    if GetVCPFeatureAndVCPFeatureReply(
                        mon.hPhysicalMonitor,
                        0xD6,
                        &mut vct as *mut MC_VCP_CODE_TYPE,
                        &mut current as *mut u32,
                        &mut max as *mut u32,
                    ) != 0
                    {
                        if SetVCPFeature(mon.hPhysicalMonitor, 0xD6, if current == 1 { 5 } else { 1 }) == 0 {
                            print_last_error("SetVCPFeature");
                        }
                    } else {
                        print_last_error("GetVCPFeatureAndVCPFeatureReply")
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

fn main() {
    if unsafe { !EnumDisplayMonitors(HDC::NULL, 0 as *mut RECT, Some(switch_proc), LPARAM::NULL).as_bool() } {
        print_last_error("EnumDisplayMonitors");
    }
}
