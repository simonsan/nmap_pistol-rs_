use log::debug;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::time::Duration;

use crate::errors::PistolErrors;
use crate::hop::icmp::send_icmp_ping_packet;
use crate::hop::icmpv6::send_icmpv6_ping_packet;

pub mod icmp;
pub mod icmpv6;

pub fn ipv4_get_hops(
    src_ipv4: Ipv4Addr,
    dst_ipv4: Ipv4Addr,
    timeout: Duration,
) -> Result<u8, PistolErrors> {
    for ttl in 1..=30 {
        let ret = send_icmp_ping_packet(src_ipv4, dst_ipv4, ttl, timeout)?;
        if ret {
            debug!("ipv4 get hops: {}", ttl);
            return Ok(ttl);
        }
    }
    Ok(0)
}

pub fn ipv6_get_hops(
    src_ipv6: Ipv6Addr,
    dst_ipv6: Ipv6Addr,
    timeout: Duration,
) -> Result<u8, PistolErrors> {
    for ttl in 1..=30 {
        let ret = send_icmpv6_ping_packet(src_ipv6, dst_ipv6, ttl, timeout)?;
        if ret {
            debug!("ipv6 get hops: {}", ttl);
            return Ok(ttl);
        }
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::find_source_addr;
    use crate::utils::find_source_addr6;
    // use crate::TEST_IPV4_REMOTE;
    use crate::TEST_IPV6_LOCAL;
    #[test]
    fn test_get_hops() {
        // use crate::Logger;
        // let _ = Logger::init_debug_logging();
        // let dst_ipv4 = Ipv4Addr::new(114, 114, 114, 114);
        let dst_ipv4 = Ipv4Addr::new(192, 168, 1, 3);
        let src_ipv4 = find_source_addr(None, dst_ipv4).unwrap();
        match src_ipv4 {
            Some(src_ipv4) => {
                let timeout = Duration::new(1, 0);
                let hops = ipv4_get_hops(src_ipv4, dst_ipv4, timeout).unwrap();
                println!("{}", hops);
            }
            None => (),
        }
    }
    #[test]
    fn test_get_hops6() {
        let src_ipv6 = find_source_addr6(None, TEST_IPV6_LOCAL).unwrap();
        match src_ipv6 {
            Some(src_ipv6) => {
                let dst_ipv6 = TEST_IPV6_LOCAL;
                let timeout = Duration::new(1, 0);
                let hops = ipv6_get_hops(src_ipv6, dst_ipv6, timeout).unwrap();
                println!("{}", hops);
            }
            None => (),
        }
    }
}
