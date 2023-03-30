use my_grpc_extensions::GrpcClientSettings;
use my_settings_reader::SettingsModel;
use serde::{Deserialize, Serialize};

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "ReportGrpc")]
    pub report_grpc: String,
}

#[async_trait::async_trait]
impl GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, _: &'static str) -> String{
        let settings = self.get_settings().await;
        settings.report_grpc.clone()
    }
}
