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
use std::{mem, usize};

static mut CURRENT: u32 = 0; // Pas de soucis...

// Obtenir la valeur du code VCP 0xD6 pour un des moniteurs
unsafe extern "system" fn current_proc(hmonitor: HMONITOR, _hdc: HDC, _rect: *mut RECT, _lparam: LPARAM) -> BOOL {
    let mut mon_count: u32 = 0;

    if GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut mon_count as *mut u32) != 0 {
        if mon_count > 0 {
            let mut mons = Vec::<PHYSICAL_MONITOR>::with_capacity(mon_count as usize);
            let mons_ptr = mons.as_mut_ptr();
            mem::forget(mons);

            if GetPhysicalMonitorsFromHMONITOR(hmonitor, mon_count, mons_ptr) != 0 {
                let mons = Vec::<PHYSICAL_MONITOR>::from_raw_parts(mons_ptr, mon_count as usize, mon_count as usize);
                let mut current: u32 = 0;
                let mut max: u32 = 0;
                let mut vct = MC_VCP_CODE_TYPE::MC_SET_PARAMETER;

                for mon in mons {
                    // Il arrive que cette fonction retourne une erreur DCC/CI
                    if GetVCPFeatureAndVCPFeatureReply(
                        mon.hPhysicalMonitor,
                        0xD6,
                        &mut vct as *mut MC_VCP_CODE_TYPE,
                        &mut current as *mut u32,
                        &mut max as *mut u32,
                    ) != 0
                    {
                        CURRENT = current;
                    } else {
                        print_last_error("GetVCPFeatureAndVCPFeatureReply"); // Erreur DCC/CI
                    }

                    if DestroyPhysicalMonitor(mon.hPhysicalMonitor) == 0 {
                        print_last_error("DestroyPhysicalMonitor");
                    }

                    if CURRENT > 0 {
                        return BOOL(0); // Succès!
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

unsafe extern "system" fn switch_proc(hmonitor: HMONITOR, _hdc: HDC, _rect: *mut RECT, lparam: LPARAM) -> BOOL {
    let mut mon_count: u32 = 0;

    if GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut mon_count as *mut u32) != 0 {
        if mon_count > 0 {
            let mut mons = Vec::<PHYSICAL_MONITOR>::with_capacity(mon_count as usize);
            let mons_ptr = mons.as_mut_ptr();
            mem::forget(mons);

            if GetPhysicalMonitorsFromHMONITOR(hmonitor, mon_count, mons_ptr) != 0 {
                let mons = Vec::<PHYSICAL_MONITOR>::from_raw_parts(mons_ptr, mon_count as usize, mon_count as usize);
                for mon in mons {
                    #[cfg(debug_assertions)]
                    print_capabilities(mon.hPhysicalMonitor);

                    if SetVCPFeature(mon.hPhysicalMonitor, 0xD6, lparam.0 as u32) == 0 {
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

unsafe fn get_current() -> u32 {
    EnumDisplayMonitors(HDC::NULL, 0 as *mut RECT, Some(current_proc), LPARAM::NULL);
    CURRENT
}

#[cfg(debug_assertions)]
use bindings::Windows::{
    Win32::Monitor::{CapabilitiesRequestAndCapabilitiesReply, GetCapabilitiesStringLength},
    Win32::SystemServices::{HANDLE, PSTR},
};
#[cfg(debug_assertions)]
unsafe fn print_capabilities(hphymon: HANDLE) {
    let mut len: u32 = 0;

    if GetCapabilitiesStringLength(hphymon, &mut len as *mut u32) != 0 {
        let mut cap = Vec::<u8>::with_capacity(len as usize);
        let cap_ptr = cap.as_mut_ptr();
        mem::forget(cap);

        if CapabilitiesRequestAndCapabilitiesReply(hphymon, PSTR(cap_ptr), len) != 0 {
            let mut cap = Vec::<u8>::from_raw_parts(cap_ptr, len as usize, len as usize);
            cap.pop(); // Enlever le nul de fin de chaîne
            println!("{}", String::from_utf8(cap).unwrap());
        } else {
            print_last_error("CapabilitiesRequestAndCapabilitiesReply");
        }
    } else {
        print_last_error("GetCapabilitiesStringLength");
    }
}

fn main() {
    unsafe {
        let new = if get_current() < 4 { 4 } else { 1 };
        if !EnumDisplayMonitors(HDC::NULL, 0 as *mut RECT, Some(switch_proc), LPARAM(new as isize)).as_bool() {
            print_last_error("EnumDisplayMonitors");
        }
    }
}
