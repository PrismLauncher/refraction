use crate::api::REQWEST_CLIENT;

use color_eyre::eyre::{eyre, Context, Result};
use log::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoryResponse {
    pub id: u64,
    pub url: String,
    pub error: Option<String>,
}

const RORY: &str = "https://rory.cat";
const ENDPOINT: &str = "/purr";

pub async fn get_rory(id: Option<u64>) -> Result<RoryResponse> {
    let target = {
        if let Some(id) = id {
            id.to_string()
        } else {
            "".to_string()
        }
    };

    let req = REQWEST_CLIENT
        .get(format!("{RORY}{ENDPOINT}/{target}"))
        .build()
        .wrap_err_with(|| "Couldn't build reqwest client!")?;

    info!("Making request to {}", req.url());
    let resp = REQWEST_CLIENT
        .execute(req)
        .await
        .wrap_err_with(|| "Couldn't make request for shiggy!")?;
    let status = resp.status();

    if let StatusCode::OK = status {
        let data = resp
            .json::<RoryResponse>()
            .await
            .wrap_err_with(|| "Couldn't parse the shiggy response!")?;

        Ok(data)
    } else {
        Err(eyre!(
            "Failed to get rory from {RORY}{ENDPOINT}/{target} with {status}",
        ))
    }
}
