#![no_std]

pub use self::ipext::{IpAdd, IpAddrRange, IpBitAnd, IpBitOr, IpSub, Ipv4AddrRange, Ipv6AddrRange};
pub use self::ipnet::{
    IpNet, IpSubnets, Ipv4Net, Ipv4Subnets, Ipv6Net, Ipv6Subnets, PrefixLenError,
};
pub use self::mask::{ip_mask_to_prefix, ipv4_mask_to_prefix, ipv6_mask_to_prefix};
pub use self::parser::AddrParseError;

mod ipext;
mod ipnet;
mod mask;
mod parser;
