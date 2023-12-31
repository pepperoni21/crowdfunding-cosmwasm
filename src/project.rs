use cosmwasm_std::{Addr, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub owner: Addr,
    pub id: String,
    pub title: String,
    pub description: String,
    pub images: Vec<String>,
    pub history: Vec<HistoryItem>,
    pub total_funding: u128,
    pub funding_goal: u128,
    pub coin: String,
}

impl Project {
    pub fn new(
        owner: Addr,
        id: String,
        title: String,
        description: String,
        images: Vec<String>,
        funding_goal: u128,
        coin: String,
    ) -> Self {
        Project {
            owner,
            id,
            title,
            description,
            images,
            history: vec![],
            total_funding: 0,
            funding_goal,
            coin,
        }
    }

    pub fn add_history_item(&mut self, funding: u128, funder: Addr, date: Timestamp) {
        self.history.push(HistoryItem {
            funding,
            funder,
            date,
        });
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HistoryItem {
    pub funding: u128,
    pub funder: Addr,
    pub date: Timestamp,
}
