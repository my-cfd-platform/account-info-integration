use std::sync::Arc;

use rust_extensions::AppStates;

use crate::{ReportGrpcClient, SettingsReader};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub report_grpc: Arc<ReportGrpcClient>,
    pub app_states: Arc<AppStates>,
}

impl AppContext {
    pub async fn new(settings: &Arc<SettingsReader>) -> Self {
        let report_grpc = Arc::new(ReportGrpcClient::new(settings.clone()));

        Self {
            report_grpc,
            app_states: Arc::new(AppStates::create_initialized()),
        }
    }
}
