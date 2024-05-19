use serde::{Deserialize, Serialize};
use mongodb::{bson::{doc, Bson}, Collection};
use chrono::{DateTime, Utc};
use bson::oid::ObjectId;

// The storage format in MongoDB.
#[derive(Debug, Serialize, Deserialize)]
pub struct NftAuction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    #[serde(rename = "auctionId")]
    pub auction_id: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "collectionAddress")]
    pub collection_address: String,
    #[serde(rename = "collectionType")]
    pub collection_type: String,
    pub quantity: u64,
    pub description: String,
    #[serde(rename = "startTime")]
    pub start_time: u64,
    #[serde(rename = "endTime")]
    pub end_time: u64,
    #[serde(rename = "startPrice")]
    pub start_price: String,
    #[serde(rename = "buyoutBidPrice")]
    pub buyout_bid_price: String,
    #[serde(rename = "currentWinner")]
    pub current_winner: String,
    #[serde(rename = "currentPrice")]
    pub current_price: String,
    #[serde(rename = "bidBufferBPS")]
    pub bid_buffer_bps: u64,
    #[serde(rename = "auctionCreatorAddress")]
    pub auction_creator_address: String,
    #[serde(rename = "timeBufferInSeconds")]
    pub time_buffer_in_seconds: u64,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "createdAt")]
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,
}

impl NftAuction {
    // pub fn new() -> Self {
    //     let now = Utc::now();
    //     Self {
    //         // ... initialize other fields ...
    //         _id: None,
    //         auction_id: "".to_string(),
    //         token_id: "".to_string(),
    //         collection_address: "".to_string(),
    //         collection_type: "".to_string(),
    //         quantity: 0,
    //         description: "".to_string(),
    //         start_time: 0,
    //         end_time: 0,
    //         start_price: "".to_string(),
    //         buyout_bid_price: "".to_string(),
    //         current_winner: "".to_string(),
    //         current_price: "".to_string(),
    //         bid_buffer_bps: 0,
    //         auction_creator_address: "".to_string(),
    //         time_buffer_in_seconds: 0,
    //         currency: "".to_string(),
    //         created_at: now,
    //         updated_at: now,
    //     }
    // }

    pub async fn save(&self, collection: &Collection<NftAuction>) -> mongodb::error::Result<()> {
        collection.insert_one(self, None).await?;
        Ok(())
    }

    pub async fn update_timestamp(&mut self, collection: &Collection<NftAuction>) -> mongodb::error::Result<()> {
        self.updated_at = Utc::now();
        collection.update_one(
            doc! { "_id": &self._id },
            doc! { "$set": { "updated_at": Bson::DateTime(self.updated_at) } },
            None
        ).await?;
        Ok(())
    }
}
