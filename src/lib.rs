#![allow(warnings)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub trait WiimoteExt {
    fn is_pressed(&self, button: u16) -> bool;
    fn is_just_pressed(&self, button: u16) -> bool;
    fn is_released(&self, button: u16) -> bool;
}

impl WiimoteExt for wiimote_t {
    fn is_pressed(&self, button: u16) -> bool {
        ((self.btns & button) != 0) || ((self.btns_held & button) != 0)
    }

    fn is_just_pressed(&self, button: u16) -> bool {
        ((self.btns & button) != 0) && ((self.btns_held & button) == 0)
    }

    fn is_released(&self, button: u16) -> bool {
        (self.btns_released & button) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_wiiuse_version_sanity() {
        unsafe {
            let version_ptr = wiiuse_version();

            assert!(
                !version_ptr.is_null(),
                "Der wiiuse_version Pointer ist null!"
            );

            let c_str = CStr::from_ptr(version_ptr);
            let version_str = c_str
                .to_str()
                .expect("wiiuse_version ist kein gültiges UTF-8");

            println!("Gefundene wiiuse Version: {}", version_str);

            assert!(
                !version_str.is_empty(),
                "Versions-String sollte nicht leer sein"
            );
        }
    }

    #[test]
    fn test_wiiuse_init_sanity() {
        unsafe {
            let wiimotes = wiiuse_init(1);

            assert!(
                !wiimotes.is_null(),
                "wiiuse_init hat einen Null-Pointer zurückgegeben"
            );

            wiiuse_cleanup(wiimotes, 1);
        }
    }
}
