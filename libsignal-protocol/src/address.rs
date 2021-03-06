use libsignal_protocol_sys as sys;
use std::{marker::PhantomData, os::raw::c_char};

pub struct Address<'a> {
    raw: sys::signal_protocol_address,
    _string_lifetime: PhantomData<&'a ()>,
}

impl<'a> Address<'a> {
    pub fn new(name: &'a str, device_id: i32) -> Address<'a> {
        let raw = sys::signal_protocol_address {
            name: name.as_ptr() as *const c_char,
            name_len: name.len(),
            device_id,
        };

        Address {
            raw,
            _string_lifetime: PhantomData,
        }
    }

    pub(crate) fn raw(&self) -> *const sys::signal_protocol_address {
        &self.raw
    }

    pub fn bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.raw.name as *const u8,
                self.raw.name_len,
            )
        }
    }

    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.bytes())
    }

    pub fn device_id(&self) -> i32 { self.raw.device_id }
}
