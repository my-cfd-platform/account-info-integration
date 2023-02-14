use std::pin::Pin;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    account_info_integration_grpc::{
        account_information_grpc_service_server::AccountInformationGrpcService,
        ActiveOrderGrpcModel, ClosedOrderGrpcModel, GetClientInfoGrpcRequest,
        PendingOrderGrpcModel, PingResponse,
    },
    GrpcService,
};

#[tonic::async_trait]
impl AccountInformationGrpcService for GrpcService {
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
        _: tonic::Request<GetClientInfoGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetActiveOrdersStream>, tonic::Status> {
        return my_grpc_extensions::grpc_server::send_vec_to_stream(vec![], |itm| itm).await;
    }

    async fn get_pending_orders(
        &self,
        _: tonic::Request<GetClientInfoGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetPendingOrdersStream>, tonic::Status> {
        return my_grpc_extensions::grpc_server::send_vec_to_stream(vec![], |itm| itm).await;
    }

    async fn get_history_positions(
        &self,
        _: tonic::Request<GetClientInfoGrpcRequest>,
    ) -> Result<tonic::Response<Self::GetHistoryPositionsStream>, tonic::Status> {
        return my_grpc_extensions::grpc_server::send_vec_to_stream(vec![], |itm| itm).await;
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
