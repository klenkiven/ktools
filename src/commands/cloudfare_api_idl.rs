use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

/// Cloudfare Update Zone Record Request
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudfareUpdateRecordRequest {
    pub content: String,
    pub name: String,
    pub proxied: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub comment: String,
    pub ttl: i64,
}

/// Cloudfare Get Zones Result
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudfareZonesResponse {
    pub result: Vec<ZonesResult>,
    #[serde(rename = "result_info")]
    pub result_info: ResultInfo,
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZonesResult {
    pub id: String,
    pub name: String,
    pub status: String,
    pub paused: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "development_mode")]
    pub development_mode: i64,
    #[serde(rename = "name_servers")]
    pub name_servers: Vec<String>,
    #[serde(rename = "original_name_servers")]
    pub original_name_servers: Vec<String>,
    #[serde(rename = "original_registrar")]
    pub original_registrar: Value,
    #[serde(rename = "original_dnshost")]
    pub original_dnshost: Value,
    #[serde(rename = "modified_on")]
    pub modified_on: String,
    #[serde(rename = "created_on")]
    pub created_on: String,
    #[serde(rename = "activated_on")]
    pub activated_on: String,
    pub owner: Owner,
    pub account: Account,
    pub tenant: Tenant,
    #[serde(rename = "tenant_unit")]
    pub tenant_unit: TenantUnit,
    pub permissions: Vec<String>,
    pub plan: Plan,
}

/// Cloudfare Get Records Result
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudfareRecordsResponse {
    pub result: Vec<RecordResult>,
    pub success: bool,
    pub errors: Vec<Value>,
    pub messages: Vec<Value>,
    #[serde(rename = "result_info")]
    pub result_info: ResultInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordResult {
    pub id: String,
    #[serde(rename = "zone_id")]
    pub zone_id: String,
    #[serde(rename = "zone_name")]
    pub zone_name: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    pub proxiable: bool,
    pub proxied: bool,
    pub ttl: i64,
    pub locked: bool,
    pub comment: Option<String>,
    pub tags: Vec<Value>,
    #[serde(rename = "created_on")]
    pub created_on: String,
    #[serde(rename = "modified_on")]
    pub modified_on: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultInfo {
    pub page: i64,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    pub count: i64,
    #[serde(rename = "total_count")]
    pub total_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub id: Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub email: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tenant {
    pub id: Value,
    pub name: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantUnit {
    pub id: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub price: i64,
    pub currency: String,
    pub frequency: String,
    #[serde(rename = "is_subscribed")]
    pub is_subscribed: bool,
    #[serde(rename = "can_subscribe")]
    pub can_subscribe: bool,
    #[serde(rename = "legacy_id")]
    pub legacy_id: String,
    #[serde(rename = "legacy_discount")]
    pub legacy_discount: bool,
    #[serde(rename = "externally_managed")]
    pub externally_managed: bool,
}

#[cfg(test)]
mod test {
    use super::CloudfareRecordsResponse;

    #[test]
    fn record_response_deserialize_test() {
        let json = "{\"result\":[{\"id\":\"f18982898528aacb06d3b5bb3c3dce5f\",\"zone_id\":\"a2736267286dc3f973a2e4f97a35c549\",\"zone_name\":\"klenkiven.xyz\",\"name\":\"sx-home.klenkiven.xyz\",\"type\":\"AAAA\",\"content\":\"2408:8226:a103:fe32:41f4:d0bb:91b4:c4c7\",\"proxiable\":true,\"proxied\":true,\"ttl\":1,\"locked\":false,\"meta\":{\"auto_added\":false,\"managed_by_apps\":false,\"managed_by_argo_tunnel\":false,\"source\":\"primary\"},\"comment\":null,\"tags\":[],\"created_on\":\"2023-04-20T13:22:38.708893Z\",\"modified_on\":\"2023-04-22T04:32:08.985413Z\"}],\"success\":true,\"errors\":[],\"messages\":[],\"result_info\":{\"page\":1,\"per_page\":100,\"count\":1,\"total_count\":1,\"total_pages\":1}}";
        let _ = serde_json::from_str::<CloudfareRecordsResponse>(json);
    }
}