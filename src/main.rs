use std::{sync::Arc, time::Duration};

use account_info_integration::{start_grpc_server, AppContext, SettingsReader};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let settings_reader = Arc::new(SettingsReader::new(".my-cfd").await);
    let app = Arc::new(AppContext::new(&settings_reader).await);
    start_grpc_server(app, 8888);

    loop {
        sleep(Duration::from_secs(100)).await;
    }
}
