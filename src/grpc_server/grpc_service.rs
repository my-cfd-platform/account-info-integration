use std::pin::Pin;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    trading_info_integration_grpc::{
        trading_info_integration_grpc_server::TradingInfoIntegrationGrpc, ActiveOrderGrpcModel,
        ClosedOrderGrpcModel, GetClientInfoGrpcRequest, PendingOrderGrpcModel, PingResponse,
    },
    GrpcService,
};

#[tonic::async_trait]
impl TradingInfoIntegrationGrpc for GrpcService {
    type GetActiveOrdersStream = Pin<
        Box<
            dyn tonic::codegen::futures_core::Stream<
                    Item = Result<ActiveOrderGrpcModel, tonic::Status>,
                > + Send
                + Sync
                + 'static,
        >,
    >;

    type GetPendingOrdersStream = Pin<
        Box<
            dyn tonic::codegen::futures_core::Stream<
                    Item = Result<PendingOrderGrpcModel, tonic::Status>,
                > + Send
                + Sync
                + 'static,
        >,
    >;

    type GetHistoryPositionsStream = Pin<
        Box<
            dyn tonic::codegen::futures_core::Stream<
                    Item = Result<ClosedOrderGrpcModel, tonic::Status>,
                > + Send
                + Sync
                + 'static,
        >,
    >;

    async fn get_active_orders(
        &self,
        request: tonic::Request<GetClientInfoGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetActiveOrdersStream>, tonic::Status> {
        let request = request.into_inner();
        let telemetry = my_telemetry::MyTelemetryContext::new();

        let from = match request.date_from {
            Some(src) => Some(src.value),
            None => None,
        };

        let to = match request.date_to {
            Some(src) => Some(src.value),
            None => None,
        };

        let result = self
            .app
            .report_grpc
            .get_active_positions(&request.account_id, from, to, &telemetry)
            .await;
        return my_grpc_extensions::grpc_server::send_vec_to_stream(result, |itm| itm).await;
    }

    async fn get_pending_orders(
        &self,
        _: tonic::Request<GetClientInfoGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetPendingOrdersStream>, tonic::Status> {
        return my_grpc_extensions::grpc_server::send_vec_to_stream(vec![], |itm| itm).await;
    }

    async fn get_history_positions(
        &self,
        request: tonic::Request<GetClientInfoGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetHistoryPositionsStream>, tonic::Status> {
        let request = request.into_inner();
        let telemetry = my_telemetry::MyTelemetryContext::new();

        let from = match request.date_from {
            Some(src) => Some(src.value),
            None => None,
        };

        let to = match request.date_to {
            Some(src) => Some(src.value),
            None => None,
        };

        let result = self
            .app
            .report_grpc
            .get_history_positions(&request.account_id, from, to, &telemetry)
            .await;

        return my_grpc_extensions::grpc_server::send_vec_to_stream(result, |itm| itm).await;
    }

    async fn ping(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<PingResponse>, tonic::Status> {
        return Ok(tonic::Response::new(PingResponse {
            service_name: "ACCOUNT_INFO_INTEGRATION".to_string(),
            date_time: DateTimeAsMicroseconds::now().unix_microseconds as u64,
        }));
    }
}
