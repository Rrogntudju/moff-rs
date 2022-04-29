use std::thread_local;
use std::{cell::Cell, ptr::null_mut, usize};
use windows::{
    Win32::Devices::Display::{
        DestroyPhysicalMonitor, GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR, GetVCPFeatureAndVCPFeatureReply,
        SetVCPFeature, MC_SET_PARAMETER, PHYSICAL_MONITOR,
    },
    Win32::Foundation::{BOOL, LPARAM, RECT},
    Win32::Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR},
};

thread_local!(static CURRENT: Cell<u32> = Cell::new(0));

// Obtenir la valeur du code VCP 0xD6 pour un des moniteurs
unsafe extern "system" fn current_proc(hmonitor: HMONITOR, _hdc: HDC, _rect: *mut RECT, _lparam: LPARAM) -> BOOL {
    let mut mon_count: u32 = 0;

    if GetNumberOfPhysicalMonitorsFromHMONITOR(hmonitor, &mut mon_count as *mut u32) != 0 {
        if mon_count > 0 {
            let mut mons = Vec::<PHYSICAL_MONITOR>::with_capacity(mon_count as usize);
            mons.set_len(mon_count as usize);
            if GetPhysicalMonitorsFromHMONITOR(hmonitor, &mut mons) != 0 {
                let (mut current, mut max) = (0, 0);
                let mut vct = MC_SET_PARAMETER;

                for mon in mons {
                    #[cfg(debug_assertions)]
                    print_capabilities(mon.hPhysicalMonitor);
                     
                    // S'il est à OFF, le moniteur peut retourner une erreur DCC/CI pour cette fonction
                    if GetVCPFeatureAndVCPFeatureReply(mon.hPhysicalMonitor, 0xD6, &mut vct, &mut current, &mut max) != 0 {
                        if current > 0 {
                            CURRENT.with(|c| c.set(current));
                            current = 0;
                        }
                    } else {
                        print_last_error("GetVCPFeatureAndVCPFeatureReply"); // Erreur DCC/CI
                    }

                    if DestroyPhysicalMonitor(mon.hPhysicalMonitor) == 0 {
                        print_last_error("DestroyPhysicalMonitor");
                    }
                    #[cfg(debug_assertions)]
                    CURRENT.with(|c| dbg!(c.get()));
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
            mons.set_len(mon_count as usize);
            if GetPhysicalMonitorsFromHMONITOR(hmonitor, &mut mons) != 0 {
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

pub fn get_d6() -> u32 {
    CURRENT.with(|c| c.set(0)); // En cas d'erreur DCC/CI
    unsafe {
        EnumDisplayMonitors(HDC(0), null_mut::<RECT>(), Some(current_proc), LPARAM(0));
    }
    CURRENT.with(|c| match c.get() {
        4 => 4, // OFF
        _ => 1, // ON
    })
}

pub fn set_d6(new: u32) {
    unsafe {
        EnumDisplayMonitors(HDC(0), null_mut::<RECT>(), Some(switch_proc), LPARAM(new as isize));
    }
}

#[cfg(debug_assertions)]
use windows::Win32::{
    Devices::Display::{CapabilitiesRequestAndCapabilitiesReply, GetCapabilitiesStringLength},
    Foundation::HANDLE,
};
#[cfg(debug_assertions)]
unsafe fn print_capabilities(hphymon: HANDLE) {
    let mut len: u32 = 0;

    // S'il est à OFF, le moniteur peut retourner une erreur DCC/CI pour cette fonction
    if GetCapabilitiesStringLength(hphymon, &mut len as *mut u32) != 0 {
        let mut cap = Vec::<u8>::with_capacity(len as usize);
        cap.set_len(len as usize);
        if CapabilitiesRequestAndCapabilitiesReply(hphymon, &mut cap) != 0 {
            cap.pop(); // Enlever le nul de fin de chaîne
            println!("{}", String::from_utf8(cap).unwrap());
        } else {
            print_last_error("CapabilitiesRequestAndCapabilitiesReply");
        }
    } else {
        print_last_error("GetCapabilitiesStringLength"); // Erreur DCC/CI
    }
}

#[cfg(test)]
mod tests {
    use super::{get_d6, set_d6};

    #[test]
    fn test_d6() {
        use std::{thread, time};

        assert_eq!(get_d6(), 1);
        set_d6(4); // OFF
        thread::sleep(time::Duration::from_millis(100));
        assert_eq!(get_d6(), 4);
        set_d6(1); // ON
    }
}
