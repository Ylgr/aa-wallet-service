use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AlchemyWebhookDto {
    pub data: AlchemyWebhookData
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AlchemyWebhookData {
    pub block: AlchemyWebhookBlock
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlchemyWebhookBlock {
    pub logs: Vec<AlchemyWebhookLog>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlchemyWebhookLog {
    pub transaction: AlchemyWebhookTransaction
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlchemyWebhookTransaction {
    pub hash: String,
    pub index: u64,
    pub from: AlchemyWebhookFrom,
    pub logs: Vec<AlchemyWebhookLogDetail>,
    #[serde(rename = "type")]
    pub _type: u8,
    pub status: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlchemyWebhookFrom {
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlchemyWebhookLogDetail {
    pub topics: Vec<String>,
    pub data: String,
}