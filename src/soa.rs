extern crate c_ares_sys;
extern crate libc;

use std::ffi::CStr;
use std::fmt;
use std::marker::PhantomData;
use std::ptr;
use std::slice;
use std::str;

use error::AresError ;
use utils::ares_error;

/// The result of a successful SOA lookup.
#[derive(Debug)]
#[allow(raw_pointer_derive)]
pub struct SOAResult {
    soa_reply: *mut c_ares_sys::Struct_ares_soa_reply,
    phantom: PhantomData<c_ares_sys::Struct_ares_soa_reply>,
}

impl SOAResult {
    /// Obtain an `SOAResult` from the response to an SOA lookup.
    pub fn parse_from(data: &[u8]) -> Result<SOAResult, AresError> {
        let mut soa_reply: *mut c_ares_sys::Struct_ares_soa_reply =
            ptr::null_mut();
        let parse_status = unsafe {
            c_ares_sys::ares_parse_soa_reply(
                data.as_ptr(),
                data.len() as libc::c_int,
                &mut soa_reply)
        };
        if parse_status != c_ares_sys::ARES_SUCCESS {
            Err(ares_error(parse_status))
        } else {
            let result = SOAResult::new(soa_reply);
            Ok(result)
        }
    }

    fn new(soa_reply: *mut c_ares_sys::Struct_ares_soa_reply) -> SOAResult {
        SOAResult {
            soa_reply: soa_reply,
            phantom: PhantomData,
        }
    }

    /// Returns the name server from this `SOAResult`.
    pub fn name_server(&self) -> &str {
        unsafe {
            let c_str = CStr::from_ptr((*self.soa_reply).nsname);
            str::from_utf8_unchecked(c_str.to_bytes())
        }
    }

    /// Returns the hostmaster from this `SOAResult`.
    pub fn hostmaster(&self) -> &str {
        unsafe {
            let c_str = CStr::from_ptr((*self.soa_reply).hostmaster);
            str::from_utf8_unchecked(c_str.to_bytes())
        }
    }

    /// Returns the serial number from this `SOAResult`.
    pub fn serial(&self) -> u32 {
        unsafe { (*self.soa_reply).serial }
    }

    /// Returns the refresh time from this `SOAResult`.
    pub fn refresh(&self) -> u32 {
        unsafe { (*self.soa_reply).refresh }
    }

    /// Returns the retry time from this `SOAResult`.
    pub fn retry(&self) -> u32 {
        unsafe { (*self.soa_reply).retry }
    }

    /// Returns the expire time from this `SOAResult`.
    pub fn expire(&self) -> u32 {
        unsafe { (*self.soa_reply).expire }
    }

    /// Returns the minimum time-to-live from this `SOAResult`.
    pub fn min_ttl(&self) -> u32 {
        unsafe { (*self.soa_reply).minttl }
    }
}

impl fmt::Display for SOAResult {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "Name server: {}, ", self.name_server()));
        try!(write!(fmt, "Hostmaster: {}, ", self.hostmaster()));
        try!(write!(fmt, "Serial: {}, ", self.serial()));
        try!(write!(fmt, "Refresh: {}, ", self.refresh()));
        try!(write!(fmt, "Retry: {}, ", self.retry()));
        try!(write!(fmt, "Expire: {}, ", self.expire()));
        try!(write!(fmt, "Minimum time-to-live: {}", self.min_ttl()));
        Ok(())
    }
}

impl Drop for SOAResult {
    fn drop(&mut self) {
        unsafe {
            c_ares_sys::ares_free_data(self.soa_reply as *mut libc::c_void);
        }
    }
}

unsafe impl Send for SOAResult { }
unsafe impl Sync for SOAResult { }

pub unsafe extern "C" fn query_soa_callback<F>(
    arg: *mut libc::c_void,
    status: libc::c_int,
    _timeouts: libc::c_int,
    abuf: *mut libc::c_uchar,
    alen: libc::c_int)
    where F: FnOnce(Result<SOAResult, AresError>) + 'static {
    let result = if status != c_ares_sys::ARES_SUCCESS {
        Err(ares_error(status))
    } else {
        let data = slice::from_raw_parts(abuf, alen as usize);
        SOAResult::parse_from(data)
    };
    let handler = Box::from_raw(arg as *mut F);
    handler(result);
}
