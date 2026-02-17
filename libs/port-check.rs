// https://github.com/ufoscout/port-check-rs/blob/master/src/lib.rs

use std::net::{
    Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener, TcpStream, ToSocketAddrs,
};
use std::ops::RangeBounds;
use std::time::Duration;

/// Represents a port for an IP address
pub enum Port {
    /// Represents a port for an IPv4 address
    Ipv4(u16),
    /// Represents a port for an IPv6 address
    Ipv6(u16),
}

impl From<u16> for Port {
    fn from(port: u16) -> Self {
        Port::Ipv4(port)
    }
}

impl Port {
    /// Creates a new IPv4 port with the specified value
    pub fn new(port: u16) -> Self {
        Port::Ipv4(port)
    }

    /// Creates a new IPv4 port with the specified value
    pub fn ipv4(port: u16) -> Self {
        Port::Ipv4(port)
    }

    /// Creates a new IPv6 port with the specified value
    pub fn ipv6(port: u16) -> Self {
        Port::Ipv6(port)
    }
}

/// Represents a port range for an IP address
pub enum Ports<R: RangeBounds<u16> + std::iter::Iterator<Item = u16>> {
    /// Represents a port range for an IPv4 address
    Ipv4(R),
    /// Represents a port range for an IPv6 address
    Ipv6(R),
}

impl<R: RangeBounds<u16> + std::iter::Iterator<Item = u16>> Ports<R> {
    /// Creates a new IPv4 port range with the specified min and max values
    pub fn new(port_range: R) -> Self {
        Self::ipv4(port_range)
    }

    /// Creates a new IPv4 port range with the specified min and max values
    pub fn ipv4(port_range: R) -> Self {
        Ports::Ipv4(port_range)
    }

    /// Creates a new Ipv6 port range with the specified min and max values
    pub fn ipv6(port_range: R) -> Self {
        Ports::Ipv6(port_range)
    }
}

impl<R: RangeBounds<u16> + std::iter::Iterator<Item = u16>> From<R> for Ports<R> {
    fn from(port_range: R) -> Self {
        Ports::Ipv4(port_range)
    }
}

/// Attempts a TCP connection to an address and returns whether it succeeded.
pub fn is_port_reachable<A: ToSocketAddrs>(address: A) -> bool {
    TcpStream::connect(address).is_ok()
}

/// Attempts a TCP connection to an address and returns whether it succeeded
pub fn is_port_reachable_with_timeout<A: ToSocketAddrs>(address: A, timeout: Duration) -> bool {
    match address.to_socket_addrs() {
        Ok(addrs) => {
            for address in addrs {
                if TcpStream::connect_timeout(&address, timeout).is_ok() {
                    return true;
                }
            }
            false
        }
        Err(_err) => false,
    }
}

/// Returns whether a port is available on the localhost
/// If the IP version is not specified, it defaults to IPv4. This happens when the port is specified as a number.
///
/// Beware that checking whether a port is open does not guarantee that it will still be open by the time you
/// attempt to use it.
/// Make sure your code is designed to handle potential time-of-check to time-of-use (TOCTOU) issues.
/// To help mitigate this, the library provides a retry-based approach via the [`with_free_port`],
/// [`with_free_ipv4_port`] and [`with_free_ipv6_port`] functions.
///
pub fn is_local_port_free<P: Into<Port>>(port: P) -> bool {
    match port.into() {
        Port::Ipv4(port) => is_local_ipv4_port_free(port),
        Port::Ipv6(port) => is_local_ipv6_port_free(port),
    }
}

/// Returns whether a port is available on the localhost for IPv4.
///  
/// Beware that checking whether a port is open does not guarantee that it will still be open by the time you
/// attempt to use it.
/// Make sure your code is designed to handle potential time-of-check to time-of-use (TOCTOU) issues.
/// To help mitigate this, the library provides a retry-based approach via the [`with_free_port`],
/// [`with_free_ipv4_port`] and [`with_free_ipv6_port`] functions.
///
pub fn is_local_ipv4_port_free(port: u16) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    TcpListener::bind(ipv4).is_ok()
}

/// Returns whether a port is available on the localhost for IPv6
///
/// Beware that checking whether a port is open does not guarantee that it will still be open by the time you
/// attempt to use it.
/// Make sure your code is designed to handle potential time-of-check to time-of-use (TOCTOU) issues.
/// To help mitigate this, the library provides a retry-based approach via the [`with_free_port`],
/// [`with_free_ipv4_port`] and [`with_free_ipv6_port`] functions.
///
pub fn is_local_ipv6_port_free(port: u16) -> bool {
    let ipv6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, port, 0, 0);
    TcpListener::bind(ipv6).is_ok()
}

/// Returns an available localhost port within the specified range.
/// If the IP version is not specified, it defaults to IPv4. This happens when the port range is specified as a range.
///
/// Beware that checking whether a port is open does not guarantee that it will still be open by the time you
/// attempt to use it.
/// Make sure your code is designed to handle potential time-of-check to time-of-use (TOCTOU) issues.
/// To help mitigate this, the library provides a retry-based approach via the [`with_free_port`],
/// [`with_free_ipv4_port`] and [`with_free_ipv6_port`] functions.
///
pub fn free_local_port_in_range<
    P: Into<Ports<R>>,
    R: RangeBounds<u16> + std::iter::Iterator<Item = u16>,
>(
    port_range: P,
) -> Option<u16> {
    match port_range.into() {
        Ports::Ipv4(port_range) => free_local_ipv4_port_in_range(port_range),
        Ports::Ipv6(port_range) => free_local_ipv6_port_in_range(port_range),
    }
}

/// Returns an available localhost port within the specified range for IPv4.
///
/// Beware that checking whether a port is open does not guarantee that it will still be open by the time you
/// attempt to use it.
/// Make sure your code is designed to handle potential time-of-check to time-of-use (TOCTOU) issues.
/// To help mitigate this, the library provides a retry-based approach via the [`with_free_port`],
/// [`with_free_ipv4_port`] and [`with_free_ipv6_port`] functions.
///
pub fn free_local_ipv4_port_in_range<R: RangeBounds<u16> + std::iter::Iterator<Item = u16>>(
    port_range: R,
) -> Option<u16> {
    port_range
        .into_iter()
        .find(|port| is_local_ipv4_port_free(*port))
}

/// Returns an available localhost port within the specified range for IPv6.
///
/// Beware that checking whether a port is open does not guarantee that it will still be open by the time you
/// attempt to use it.
/// Make sure your code is designed to handle potential time-of-check to time-of-use (TOCTOU) issues.
/// To help mitigate this, the library provides a retry-based approach via the [`with_free_port`],
/// [`with_free_ipv4_port`] and [`with_free_ipv6_port`] functions.
///
pub fn free_local_ipv6_port_in_range<R: RangeBounds<u16> + std::iter::Iterator<Item = u16>>(
    port_range: R,
) -> Option<u16> {
    port_range
        .into_iter()
        .find(|port| is_local_ipv6_port_free(*port))
}

/// Returns an available localhost port for IPv4.
///
/// Beware that checking whether a port is open does not guarantee that it will still be open by the time you
/// attempt to use it.
/// Make sure your code is designed to handle potential time-of-check to time-of-use (TOCTOU) issues.
/// To help mitigate this, the library provides a retry-based approach via the [`with_free_port`],
/// [`with_free_ipv4_port`] and [`with_free_ipv6_port`] functions.
///
pub fn free_local_port() -> Option<u16> {
    free_local_ipv4_port()
}

/// Returns an available localhost port for IPv4.
///
/// Beware that checking whether a port is open does not guarantee that it will still be open by the time you
/// attempt to use it.
/// Make sure your code is designed to handle potential time-of-check to time-of-use (TOCTOU) issues.
/// To help mitigate this, the library provides a retry-based approach via the [`with_free_port`],
/// [`with_free_ipv4_port`] and [`with_free_ipv6_port`] functions.
///
pub fn free_local_ipv4_port() -> Option<u16> {
    let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
    TcpListener::bind(socket)
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .ok()
}

/// Returns an available localhost port for IPv6.
///
/// Beware that checking whether a port is open does not guarantee that it will still be open by the time you
/// attempt to use it.
/// Make sure your code is designed to handle potential time-of-check to time-of-use (TOCTOU) issues.
/// To help mitigate this, the library provides a retry-based approach via the [`with_free_port`],
/// [`with_free_ipv4_port`] and [`with_free_ipv6_port`] functions.
///
pub fn free_local_ipv6_port() -> Option<u16> {
    let socket = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 0, 0, 0);
    TcpListener::bind(socket)
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .ok()
}

/// Alias for [`with_free_ipv4_port`]-
///
/// Attempts to find a free IPv4 port on localhost and applies the provided function `f` to it.
///
/// This function iterates over available ports on the localhost IPv4 interface,
/// invoking the passed closure `f` with the port number. If the closure returns
/// an `Ok` result, the function returns `Some` with the result. If no suitable
/// port is found or the closure does not return `Ok`, it returns `None`.
///
/// This function is usefull to deal with the TOC/TOU race condition, where
/// the port might be used by another process before it is bound to a socket.
///
/// # Arguments
///
/// - `f`: A closure that is applied to each available port until it returns `Ok`.
///
/// # Returns
///
/// An `Option` containing the successful result of the closure and the used port, or `None` if no port
/// was suitable or the closure never returned `Ok`.
///
/// # Example
///
/// ```
/// use port_check::*;
/// use std::net::*;
///
/// let (server, port) = with_free_ipv4_port::<_, std::io::Error, _>(|port| {
///     // Trying to use the port. If it fails, we try again with another port
///     let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))?;
///     Ok(listener)
/// }).unwrap();
/// ```
#[inline]
pub fn with_free_port<T, E, F: Fn(u16) -> Result<T, E>>(f: F) -> Option<(T, u16)> {
    with_free_ipv4_port(f)
}

/// Attempts to find a free IPv4 port on localhost and applies the provided function `f` to it.
///
/// This function delegates to `with_free_port`, using the IPv4 interface to search for available ports.
/// It invokes the given closure `f` with each available port. If the closure returns an `Ok` result,
/// the function returns `Some` with this result. If no suitable port is found or the closure never
/// returns `Ok`, it returns `None`.
///
/// This function helps mitigate TOC/TOU race conditions by ensuring that the port is bound
/// immediately after being checked for availability.
///
/// # Arguments
///
/// - `f`: A closure that is applied to each available IPv4 port until it returns `Ok`.
///
/// # Returns
///
/// An `Option` containing the successful result of the closure and the used port, or `None` if no port
/// was suitable or the closure never returned `Ok`.
///
/// # Example
///
/// ```
/// use port_check::*;
/// use std::net::*;
///
/// let (server, port) = with_free_ipv4_port::<_, std::io::Error, _>(|port| {
///     // Trying to use the port. If it fails, we try again with another port
///     let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))?;
///     Ok(listener)
/// }).unwrap();
/// ```
pub fn with_free_ipv4_port<T, E, F: Fn(u16) -> Result<T, E>>(f: F) -> Option<(T, u16)> {
    while let Some(port) = free_local_ipv4_port() {
        if let Ok(value) = f(port) {
            return Some((value, port));
        }
    }
    None
}

/// Attempts to find a free IPv6 port on localhost and applies the provided function `f` to it.
///
/// This function iterates over available ports on the localhost IPv6 interface,
/// invoking the passed closure `f` with the port number. If the closure returns
/// an `Ok` result, the function returns `Some` with the result. If no suitable
/// port is found or the closure does not return `Ok`, it returns `None`.
///
/// This function is usefull to deal with the TOC/TOU race condition, where
/// the port might be used by another process before it is bound to a socket.
///
/// # Arguments
///
/// - `f`: A closure that is applied to each available port until it returns `Ok`.
///
/// # Returns
///
/// An `Option` containing the successful result of the closure and the used port, or `None` if no port
/// was suitable or the closure never returned `Ok`.
///
/// # Example
///
/// ```
/// use port_check::*;
/// use std::net::*;
///
/// let (server, port) = with_free_ipv6_port::<_, std::io::Error, _>(|port| {
///     // Trying to use the port. If it fails, we try again with another port
///     let listener = TcpListener::bind(SocketAddrV6::new(Ipv6Addr::LOCALHOST, port, 0, 0))?;
///     Ok(listener)
/// }).unwrap();
/// ```
pub fn with_free_ipv6_port<T, E, F: Fn(u16) -> Result<T, E>>(f: F) -> Option<(T, u16)> {
    while let Some(port) = free_local_ipv6_port() {
        if let Ok(value) = f(port) {
            return Some((value, port));
        }
    }
    None
}
