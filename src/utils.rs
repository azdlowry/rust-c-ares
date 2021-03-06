use c_types;
use c_ares_sys;

use types::AddressFamily;
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_int;
use std::net::{
    Ipv4Addr,
    Ipv6Addr,
    SocketAddrV4,
    SocketAddrV6,
};
use std::str;

// Convert an address family into a more strongly typed AddressFamily.
pub fn address_family(family: c_int) -> Option<AddressFamily> {
    match family {
        c_types::AF_INET => Some(AddressFamily::INET),
        c_types::AF_INET6 => Some(AddressFamily::INET6),
        c_types::AF_UNSPEC => Some(AddressFamily::UNSPEC),
        _ => None,
    }
}

// Get an in_addr from an Ipv4Addr.
#[cfg(unix)]
pub fn ipv4_as_in_addr(ipv4: &Ipv4Addr) -> c_types::in_addr {
    c_types::in_addr { s_addr: u32::from(*ipv4).to_be() }
}

#[cfg(windows)]
pub fn ipv4_as_in_addr(ipv4: &Ipv4Addr) -> c_types::in_addr {
    c_types::in_addr { S_un: u32::from(*ipv4).to_be() }
}

// Get an Ipv4Addr from an in_addr.
#[cfg(unix)]
pub fn ipv4_from_in_addr(in_addr: &c_types::in_addr) -> Ipv4Addr {
    Ipv4Addr::from(u32::from_be(in_addr.s_addr))
}

#[cfg(windows)]
pub fn ipv4_from_in_addr(in_addr: &c_types::in_addr) -> Ipv4Addr {
    Ipv4Addr::from(u32::from_be(in_addr.S_un))
}

// Get an in6_addr from an Ipv6Addr.
pub fn ipv6_as_in6_addr(ipv6: &Ipv6Addr) -> c_types::in6_addr {
    let segments = ipv6.segments();
    let mut in6_addr: c_types::in6_addr = unsafe { mem::uninitialized() };
    in6_addr.s6_addr[0] = (segments[0] >> 8) as u8;
    in6_addr.s6_addr[1] = segments[0] as u8;
    in6_addr.s6_addr[2] = (segments[1] >> 8) as u8;
    in6_addr.s6_addr[3] = segments[1] as u8;
    in6_addr.s6_addr[4] = (segments[2] >> 8) as u8;
    in6_addr.s6_addr[5] = segments[2] as u8;
    in6_addr.s6_addr[6] = (segments[3] >> 8) as u8;
    in6_addr.s6_addr[7] = segments[3] as u8;
    in6_addr.s6_addr[8] = (segments[4] >> 8) as u8;
    in6_addr.s6_addr[9] = segments[4] as u8;
    in6_addr.s6_addr[10] = (segments[5] >> 8) as u8;
    in6_addr.s6_addr[11] = segments[5] as u8;
    in6_addr.s6_addr[12] = (segments[6] >> 8) as u8;
    in6_addr.s6_addr[13] = segments[6] as u8;
    in6_addr.s6_addr[14] = (segments[7] >> 8) as u8;
    in6_addr.s6_addr[15] = segments[7] as u8;
    in6_addr
}

// Get a sockaddr_in from a SocketAddrV4.
#[cfg(any(target_os = "macos",
          target_os = "ios",
          target_os = "freebsd",
          target_os = "dragonfly",
          target_os = "openbsd",
          target_os = "netbsd",
          target_os = "bitrig"))]
pub fn socket_addrv4_as_sockaddr_in(
    sock_v4: &SocketAddrV4) -> c_types::sockaddr_in {
    let in_addr = ipv4_as_in_addr(sock_v4.ip());
    c_types::sockaddr_in {
        sin_len: mem::size_of::<c_types::sockaddr_in>() as u8,
        sin_family: c_types::AF_INET as c_types::sa_family_t,
        sin_port: sock_v4.port().to_be(),
        sin_addr: in_addr,
        sin_zero: [0; 8],
    }
}

#[cfg(not(any(target_os = "macos",
              target_os = "ios",
              target_os = "freebsd",
              target_os = "dragonfly",
              target_os = "openbsd",
              target_os = "netbsd",
              target_os = "bitrig")))]
pub fn socket_addrv4_as_sockaddr_in(
    sock_v4: &SocketAddrV4) -> c_types::sockaddr_in {
    let in_addr = ipv4_as_in_addr(sock_v4.ip());
    c_types::sockaddr_in {
        sin_family: c_types::AF_INET as c_types::sa_family_t,
        sin_port: sock_v4.port().to_be(),
        sin_addr: in_addr,
        sin_zero: [0; 8],
    }
}

// Get a sockaddr_in6 from a SocketAddrV6.
#[cfg(any(target_os = "macos",
          target_os = "ios",
          target_os = "freebsd",
          target_os = "dragonfly",
          target_os = "openbsd",
          target_os = "netbsd",
          target_os = "bitrig"))]
pub fn socket_addrv6_as_sockaddr_in6(
    sock_v6: &SocketAddrV6) -> c_types::sockaddr_in6 {
    let in6_addr = ipv6_as_in6_addr(sock_v6.ip());
    c_types::sockaddr_in6 {
        sin6_len: mem::size_of::<c_types::sockaddr_in6>() as u8,
        sin6_family: c_types::AF_INET6 as c_types::sa_family_t,
        sin6_port: sock_v6.port().to_be(),
        sin6_addr: in6_addr,
        sin6_flowinfo: sock_v6.flowinfo(),
        sin6_scope_id: sock_v6.scope_id(),
    }
}

#[cfg(all(unix,not(any(target_os = "macos",
                       target_os = "ios",
                       target_os = "freebsd",
                       target_os = "dragonfly",
                       target_os = "openbsd",
                       target_os = "netbsd",
                       target_os = "bitrig"))))]
pub fn socket_addrv6_as_sockaddr_in6(
    sock_v6: &SocketAddrV6) -> c_types::sockaddr_in6 {
    let in6_addr = ipv6_as_in6_addr(sock_v6.ip());
    c_types::sockaddr_in6 {
        sin6_family: c_types::AF_INET6 as c_types::sa_family_t,
        sin6_port: sock_v6.port().to_be(),
        sin6_addr: in6_addr,
        sin6_flowinfo: sock_v6.flowinfo(),
        sin6_scope_id: sock_v6.scope_id(),
    }
}

#[cfg(windows)]
pub fn socket_addrv6_as_sockaddr_in6(
    sock_v6: &SocketAddrV6) -> c_types::sockaddr_in6 {
    let in6_addr = ipv6_as_in6_addr(sock_v6.ip());
    c_types::sockaddr_in6 {
        sin6_family: c_types::AF_INET6 as i16,
        sin6_port: sock_v6.port().to_be(),
        sin6_addr: in6_addr,
        sin6_flowinfo: sock_v6.flowinfo(),
        sin6_scope_id: sock_v6.scope_id(),
    }
}

/// Get the version number of the underlying `c-ares` library.
///
/// The version is returned as both a string and an integer.  The integer is
/// built up as 24bit number, with 8 separate bits used for major number, minor
/// number and patch number.  For example, the version string "1.2.3" is
/// returned as hexadecimal number 0x010203 (decimal 66051).
pub fn version() -> (&'static str, u32) {
    let mut int_version: c_int = 0;
    let str_version = unsafe {
        let ptr = c_ares_sys::ares_version(&mut int_version);
        let buf = CStr::from_ptr(ptr).to_bytes();
        str::from_utf8_unchecked(buf)
    };
    (str_version, int_version as u32)
}
