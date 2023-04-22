use std::{fs, io, process};

use anyhow::{anyhow, Error, Ok};
use lazy_static::lazy_static;

use crate::args::RecordType;

use super::cloudfare_api_idl::{CloudfareZonesResponse, CloudfareRecordsResponse, CloudfareUpdateRecordRequest};

lazy_static! {
    /// Home directory
    static ref HOME_DIRECTORY: String = std::env::var("HOME").unwrap_or_else(|_| String::from("~"));
}
/// Cloudfare ddns tools
pub fn cloudfare_ddns(
    token: &str,
    zone_name: &str,
    record_name: &str,
    record_type: &RecordType,
    ttl: &i32,
    proxied: &bool,
    force: &bool,
) -> Result<(), Error> {
    // 1. Get the IP address
    let current_ip = get_ip_address(record_type)?;
    tracing::info!("Current WAN IP [{}]", &current_ip);

    // 2. Check FQDN Record
    let record_name = check_fqdn_record(zone_name, record_name)?;

    // 3. Get old WAN IP
    let old_wan_ip = get_old_wan_ip(record_name);

    // 4. If WAN IP is not changed, print log
    if !force && current_ip.eq_ignore_ascii_case(&old_wan_ip) {
        tracing::info!("WAN IP [{}] Unchanged, to update anyway use flag -f true", old_wan_ip);
        process::exit(0);
    }

    // 5. Get zone_identifier & record_identifier
    let zone_identifier = get_zone_identifier(token, zone_name)?;
    let record_identifier = get_record_identifier(token, &zone_identifier, record_name)?;

    // 6. If WAN is changed, update cloudflare
    let request = CloudfareUpdateRecordRequest {
        content: current_ip,
        name: record_name.to_string(),
        proxied: *proxied,
        comment: String::from("DDNS"),
        ttl: *ttl as i64,
        type_field: match record_type {
            RecordType::A => "A".to_string(),
            RecordType::AAAA => "AAAA".to_string(),
        },
    };
    update_if_necessary(token, &zone_identifier, &record_identifier, request)?;

    Ok(())
}

/// Get IP Address with ipw.cn
fn get_ip_address(record_type: &RecordType) -> Result<String, Error> {
    let request_url = match record_type {
        RecordType::A => "http://4.ipw.cn",
        RecordType::AAAA => "http://6.ipw.cn"
    };

    let response = minreq::get(request_url).send()?;
    let ip = response
            .as_str()
            .map_err(|e| anyhow!("Get IP address error: {}", e));
    Ok(ip.unwrap().to_string())
}

/// Check FQDN Record
fn check_fqdn_record<'a>(zone_name: &str, record_name: &'a str) -> Result<&'a str, Error> {
    if zone_name != record_name && !record_name.ends_with(zone_name) {
        return Err(anyhow!("Hostname is not a FQDN, please try again"));
    }
    Ok(record_name)
}

// Get old WAN ip
fn get_old_wan_ip(record_name: &str) -> String {
    let filename = format!("{}/.cf_old_wan_ip_{}.txt", HOME_DIRECTORY.as_str(), record_name);

    match fs::read_to_string(filename) {
        io::Result::Ok(s) => s,
        Err(_) => String::new()
    }
}

/// Get zone_identifier
fn get_zone_identifier(token: &str, zone_name: &str) -> Result<String, Error> {
    // Do request get the response
    let response = minreq::get(format!("https://api.cloudflare.com/client/v4/zones?name={}", zone_name))
            .with_header("Authorization", token)
            .with_header("Content-Type", "application/json")
            .send()?;

    // Deserialize the response
    let body = response.as_str()?;
    tracing::debug!("Zone [{}] result is {:?}", zone_name, &body);
    let body = serde_json::from_str::<CloudfareZonesResponse>(body)
            .map_err(|e| anyhow!("CloudfareZonesResponse Json parse error: {}", e))?;
    tracing::info!("Zone [{}] identifier is {:?}", zone_name, &body.result[0].id);

    Ok(body.result[0].id.clone())
}

/// Get record_identifier
fn get_record_identifier(token: &str, zone_id: &str, record_name: &str) -> Result<String, Error> {
    // Do request get the response
    let response = minreq::get(format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}", zone_id, record_name))
            .with_header("Authorization", token)
            .with_header("Content-Type", "application/json")
            .send()?;

    // Deserialize the response
    let body = response.as_str()?;
    tracing::debug!("Record [{}] result is {}", record_name, &body);
    let body = serde_json::from_str::<CloudfareRecordsResponse>(body)
            .map_err(|e| anyhow!("CloudfareRecordsResponse Json parse error: {}", e))?;
    
    // Get record identifier
    let record_identifier = if body.result.is_empty() {
        String::new()
    } else {
        body.result[0].id.clone()
    };
    tracing::info!("Record [{}] identifier is {:?}", record_name, record_identifier);

    Ok(record_identifier)
}

/// If WAN is changed, update cloudflare
fn update_if_necessary(
    token: &str, zone_identifier: &str, record_identifier: &str, 
    body: CloudfareUpdateRecordRequest
) -> Result<(), Error> {
    // Construct the request
    tracing::debug!("DNS Update Request: {:?}", &body);
    let update_url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", zone_identifier, record_identifier);
    let request = if record_identifier.is_empty() {
        minreq::post(update_url)
    } else {
        minreq::put(update_url)
    };

    // Do request get the response
    let response = request
            .with_header("Authorization", token)
            .with_header("Content-Type", "application/json")
            .with_body(serde_json::to_string(&body)?)
            .send()?;

    tracing::info!("DNS Update: record_name={}, ip={}, proxied={}, ttl={}", 
            &body.name, &body.content, &body.proxied, &body.ttl);
    tracing::debug!("DNS Update: result={}", response.as_str()?);

    Ok(())
}
