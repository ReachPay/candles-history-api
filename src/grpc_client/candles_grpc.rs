use std::time::Duration;

use my_telemetry::MyTelemetryContext;
use prost::encoding::bool;
use tonic::{transport::Channel, codegen::InterceptedService};

use crate::{MyInterceptor, orders_grpc::{simple_trading_engine_grpc_service_client::SimpleTradingEngineGrpcServiceClient, GetAllFromCacheGrpcRequest, CacheCandleGrpcModel}};

pub struct OrdersFlowsGrpcClient {
    channel: Channel,
    timeout: Duration,
}

impl OrdersFlowsGrpcClient {
    pub async fn new(grpc_address: String) -> Self {
        let channel = Channel::from_shared(grpc_address)
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            channel,
            timeout: Duration::from_secs(5),
        }
    }

    async fn create_grpc_service(
        &self,
        my_telemetry_context: &MyTelemetryContext,
    ) -> SimpleTradingEngineGrpcServiceClient<InterceptedService<Channel, MyInterceptor>> {
        let client: SimpleTradingEngineGrpcServiceClient<InterceptedService<Channel, MyInterceptor>> =
        SimpleTradingEngineGrpcServiceClient::with_interceptor(
                self.channel.clone(),
                MyInterceptor::new(my_telemetry_context),
            );

        client
    }

    pub async fn get_all_candles(&self, is_bid: bool, my_telemetry_context: &MyTelemetryContext) -> Vec<CacheCandleGrpcModel>{
        let mut client = self.create_grpc_service(my_telemetry_context).await;

        let result = client.get_all_from_cache(GetAllFromCacheGrpcRequest{
            is_bids: is_bid,
            source: "ST".to_string(),
        }).await.unwrap();

        let mut inner = result.into_inner();
        let mut result = Vec::new();

        while let Some(feature) = inner.message().await.unwrap() {
            result.push(feature);
        }

        return result;
    }
}