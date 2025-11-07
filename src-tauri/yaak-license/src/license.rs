use crate::error::Error::{ClientError, ServerError};
use crate::error::Result;
use chrono::{NaiveDateTime, Utc};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, Runtime, WebviewWindow, is_dev};
use ts_rs::TS;
use yaak_common::api_client::yaak_api_client;
use yaak_common::platform::get_os;
use yaak_models::query_manager::QueryManagerExt;
use yaak_models::util::UpdateSource;

const KV_NAMESPACE: &str = "license";
const KV_ACTIVATION_ID_KEY: &str = "activation_id";
const TRIAL_SECONDS: u64 = 3600 * 24 * 30;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "gen_models.ts")]
pub struct CheckActivationRequestPayload {
    pub app_version: String,
    pub app_platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "license.ts")]
pub struct CheckActivationResponsePayload {
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "license.ts")]
pub struct ActivateLicenseRequestPayload {
    pub license_key: String,
    pub app_version: String,
    pub app_platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "license.ts")]
pub struct DeactivateLicenseRequestPayload {
    pub app_version: String,
    pub app_platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "license.ts")]
pub struct ActivateLicenseResponsePayload {
    pub activation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "license.ts")]
pub struct APIErrorResponsePayload {
    pub error: String,
    pub message: String,
}

pub async fn activate_license<R: Runtime>(
    _window: &WebviewWindow<R>,
    _license_key: &str,
) -> Result<()> {
    Ok(())
}

pub async fn deactivate_license<R: Runtime>(_window: &WebviewWindow<R>) -> Result<()> {
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case", tag = "type")]
#[ts(export, export_to = "license.ts")]
pub enum LicenseCheckStatus {
    PersonalUse { trial_ended: NaiveDateTime },
    CommercialUse,
    InvalidLicense,
    Trialing { end: NaiveDateTime },
}

pub async fn check_license<R: Runtime>(_window: &WebviewWindow<R>) -> Result<LicenseCheckStatus> {
    Ok(LicenseCheckStatus::CommercialUse)
}

fn build_url(path: &str) -> String {
    if is_dev() {
        format!("http://localhost:9444{path}")
    } else {
        format!("https://license.yaak.app{path}")
    }
}

pub async fn get_activation_id<R: Runtime>(_app_handle: &AppHandle<R>) -> String {
    "PERM-COMMERCIAL".to_string()
}
