mod cloudfare_ddns;
mod cloudfare_api_idl;

use std::process;

use crate::args::RecordType;

/// Cloudfare ddns tools
use crate::commands::cloudfare_ddns::cloudfare_ddns;
pub fn cf_ddns(
    token: &str,
    zone_name: &str,
    record_name: &str,
    record_type: &RecordType,
    ttl: &i32,
    proxied: &bool,
    force: &bool,
) {
    let result = cloudfare_ddns(token, zone_name, record_name, record_type, ttl, proxied, force);
    if let Err(e) = result {
        tracing::error!("Error: {}", e);
        // If some Error occured, exit with code 2
        process::exit(2);
    };
}
