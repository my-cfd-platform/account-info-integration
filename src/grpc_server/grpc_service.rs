use std::pin::Pin;

use crate::{
    trading_info_integration_grpc::{
        trading_information_grpc_service_server::TradingInformationGrpcService,
        ActiveOrderGrpcModel, ClosedOrderGrpcModel, GetClientInfoGrpcRequest,
        PendingOrderGrpcModel,
    },
    GrpcService,
};

#[tonic::async_trait]
impl TradingInformationGrpcService for GrpcService {
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
}
