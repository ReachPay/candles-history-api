mod candle_history_cache;
mod save_queue;
mod models;

pub use candle_history_cache::*;
pub use save_queue::*;
pub use models::*;



pub fn trunc_date(date: u64, candle_type: CandleType) -> u64{
    match candle_type {
        CandleType::Minute => date - date % 60,
        CandleType::Hour => date - date % 3600,
        CandleType::Day => date - date % 86400,
        CandleType::Month => date - date % 2592000,
    }
}