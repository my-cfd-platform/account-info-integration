use std::{sync::Arc, time::Duration};

use my_grpc_extensions::{GrpcChannel, GrpcClientInterceptor, GrpcClientSettings};
use my_telemetry::MyTelemetryContext;
use tonic::{codegen::InterceptedService, transport::Channel};

use crate::{
    report_grpc::{
        report_grpc_service_client::ReportGrpcServiceClient,
        ReportFlowsOperationsGetHistoryPaginatedGrpcResponse,
        ReportFlowsOperationsGetInDateRangeGrpcRequest,
        ReportFlowsOperationsGetPaginatedGrpcRequest,
    },
    trading_info_integration_grpc::{ActiveOrderGrpcModel, ClosedOrderGrpcModel},
};

pub const REPORT_GRPC_SERVICE_NAME: &str = "report_grpc_service";

pub struct ReportGrpcClient {
    grpc_channel: GrpcChannel,
}

impl ReportGrpcClient {
    pub fn new(get_grpc_address: Arc<dyn GrpcClientSettings + Send + Sync + 'static>) -> Self {
        Self {
            grpc_channel: GrpcChannel::new(
                get_grpc_address,
                REPORT_GRPC_SERVICE_NAME,
                Duration::from_secs(10),
            ),
        }
    }

    async fn create_grpc_service(
        &self,
        ctx: &MyTelemetryContext,
    ) -> ReportGrpcServiceClient<InterceptedService<Channel, GrpcClientInterceptor>> {
        let client: ReportGrpcServiceClient<InterceptedService<Channel, GrpcClientInterceptor>> =
            ReportGrpcServiceClient::with_interceptor(
                self.grpc_channel.get_channel().await.unwrap(),
                GrpcClientInterceptor::new(ctx.clone()),
            );

        client
    }

    pub async fn get_operation_history(
        &self,
        account_id: &str,
        page: i32,
        size: i32,
        ctx: &MyTelemetryContext,
    ) -> ReportFlowsOperationsGetHistoryPaginatedGrpcResponse {
        let mut grpc_client = self.create_grpc_service(ctx).await;

        let result = grpc_client
            .get_history_paginated(ReportFlowsOperationsGetPaginatedGrpcRequest {
                account_id: account_id.to_string(),
                page,
                size,
            })
            .await
            .unwrap();

        return result.into_inner();
    }
    pub async fn get_active_positions(
        &self,
        account_id: &str,
        from: Option<u64>,
        to: Option<u64>,
        ctx: &MyTelemetryContext,
    ) -> Vec<ActiveOrderGrpcModel> {
        let mut grpc_client = self.create_grpc_service(ctx).await;

        let result = grpc_client
            .get_active_positions_in_date_range(ReportFlowsOperationsGetInDateRangeGrpcRequest {
                account_id: account_id.to_string(),
                date_from: from,
                date_to: to,
            })
            .await
            .unwrap()
            .into_inner();

        let positions =
            my_grpc_extensions::read_grpc_stream::as_vec(result, Duration::from_secs(20))
                .await
                .unwrap();

        let positions = match positions {
            Some(pos) => pos,
            None => vec![],
        };

        let positions = positions.iter().map(|x| x.to_owned().into()).collect();

        return positions;
    }

    pub async fn get_history_positions(
        &self,
        account_id: &str,
        from: Option<u64>,
        to: Option<u64>,
        ctx: &MyTelemetryContext,
    ) -> Vec<ClosedOrderGrpcModel> {
        let mut grpc_client = self.create_grpc_service(ctx).await;

        let result = grpc_client
            .get_history_positions_in_date_range(ReportFlowsOperationsGetInDateRangeGrpcRequest {
                account_id: account_id.to_string(),
                date_from: from,
                date_to: to,
            })
            .await
            .unwrap()
            .into_inner();

        let positions =
            my_grpc_extensions::read_grpc_stream::as_vec(result, Duration::from_secs(20))
                .await
                .unwrap();

        let positions = match positions {
            Some(pos) => pos,
            None => vec![],
        };

        let positions = positions.iter().map(|x| x.to_owned().into()).collect();

        return positions;
    }
}
