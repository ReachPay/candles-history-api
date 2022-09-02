use candles_history_api::{OrdersFlowsGrpcClient, CandleHistoryCache, CandleType, Candle, AppContext, SettingsModel};
use my_telemetry::MyTelemetryContext;


#[tokio::main]
async fn main() {
    // let settings = SettingsModel::load(".reachpay").await;
    let grpc = OrdersFlowsGrpcClient::new("http://192.168.1.9:5961".to_string()).await;
    let candles = grpc.get_all_candles(true, &MyTelemetryContext::new()).await;

    let cache = CandleHistoryCache::new();

    for candle in candles{
        println!("{:?}", candle);
        let candle_model = candle.candle.unwrap();
        cache.init(candle.instrument_id, true, 
            CandleType::parse(candle.candle_type as u8), 
            Candle { open: candle_model.open, close: candle_model.close, high: candle_model.high, low: candle_model.low, volume: 0.0, timestamp: candle_model.date_time }).await;
    }   

    let app = AppContext::new_with_cache(cache);
    app.app_states.wait_until_shutdown().await;
}