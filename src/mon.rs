mod bindings {
    windows::include_bindings!();
}

use bindings::Windows::{
    Win32::Devices::Display::{
        DestroyPhysicalMonitor, GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR, GetVCPFeatureAndVCPFeatureReply,
        SetVCPFeature, MC_SET_PARAMETER, PHYSICAL_MONITOR,
    },
    Win32::Foundation::{BOOL, LPARAM, RECT},
    Win32::Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
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
                let mut vct = MC_SET_PARAMETER;

                for mon in mons {
                    #[cfg(debug_assertions)]
                    print_capabilities(mon.hPhysicalMonitor);

                    // Il arrive que cette fonction retourne une erreur DCC/CI
                    if GetVCPFeatureAndVCPFeatureReply(mon.hPhysicalMonitor, 0xD6, &mut vct, &mut current, &mut max) != 0 {
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

unsafe extern "system" fn switch_proc(hmonitor: HMONITOR, _hdc: HDC, _rect: *mut RECT, LPARAM(new): LPARAM) -> BOOL {
    let mut mon_count: u32 = 0;

    if GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut mon_count as *mut u32) != 0 {
        if mon_count > 0 {
            let mut mons = Vec::<PHYSICAL_MONITOR>::with_capacity(mon_count as usize);
            let mons_ptr = mons.as_mut_ptr();
            mem::forget(mons);

            if GetPhysicalMonitorsFromHMONITOR(hmonitor, mon_count, mons_ptr) != 0 {
                let mons = Vec::<PHYSICAL_MONITOR>::from_raw_parts(mons_ptr, mon_count as usize, mon_count as usize);
                for mon in mons {
                    if SetVCPFeature(mon.hPhysicalMonitor, 0xD6, new as u32) == 0 {
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

pub fn get_current_d6() -> Option<u32> {
    unsafe {
        CURRENT = 0;
        EnumDisplayMonitors(HDC::NULL, 0 as *mut RECT, Some(current_proc), LPARAM::NULL);
        match CURRENT {
            0 => None,
            4 => Some(4), // OFF
            _ => Some(1), // ON
        }
    }
}

pub fn set_d6(new: u32) {
    unsafe {
        if !EnumDisplayMonitors(HDC::NULL, 0 as *mut RECT, Some(switch_proc), LPARAM(new as isize)).as_bool() {
            print_last_error("EnumDisplayMonitors");
        }
    }
}

#[cfg(debug_assertions)]
use bindings::Windows::Win32::{
    Devices::Display::{CapabilitiesRequestAndCapabilitiesReply, GetCapabilitiesStringLength},
    Foundation::{HANDLE, PSTR},
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

#[cfg(test)]
mod tests {
    use super::get_current_d6;

    #[test]
    fn test_get_current_d6() {
        assert_eq!(get_current_d6(), Some(1));
    }
}
