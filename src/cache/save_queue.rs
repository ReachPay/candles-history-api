use tokio::sync::{Mutex};

use crate::{CandleType, Candle};

pub struct CandlesCacheItem{
    pub instrument_id: String,
    pub candle_type: CandleType,
    pub candle: Candle,
    pub date: u64
}

pub struct CandleCacheSaveQueue{
    pub bids_to_save: Mutex<Vec<CandlesCacheItem>>,
    pub asks_to_save: Mutex<Vec<CandlesCacheItem>>,
}

impl CandleCacheSaveQueue {

    pub fn new() -> Self {
        Self{
            bids_to_save: Mutex::new(Vec::new()),
            asks_to_save: Mutex::new(Vec::new()),
        }
    }

    pub async fn save_candle(&mut self, instrument_id: &str, is_bid: bool, candle_type: CandleType, candle: Candle){
        let cache_item = CandlesCacheItem{
            instrument_id: instrument_id.to_string(),
            candle_type: candle_type,
            candle: candle.clone(),
            date: candle.timestamp
        };

        match is_bid {
            true => self.asks_to_save.lock().await.push(cache_item),
            false => self.asks_to_save.lock().await.push(cache_item),
        }
    }

    pub async fn get(&self) -> (Option<CandlesCacheItem>, Option<CandlesCacheItem>){
        let mut bid_to_return : Option<CandlesCacheItem> = None;
        let mut ask_to_return : Option<CandlesCacheItem> = None;
        {
            let mut bids_lock = self.bids_to_save.lock().await;
            if &bids_lock.len() > &0 {
                bid_to_return = Some(bids_lock.remove(0));
            }
        }

        {
            let mut asks_lock = self.bids_to_save.lock().await;
            if &asks_lock.len() > &0 {
                ask_to_return = Some(asks_lock.remove(0));
            }
        }

        return (bid_to_return, ask_to_return);
    }
}


// pub struct CandleCacheItem{
//     pub minute_candles: RwLock<BTreeMap<u64, Candle>>,
//     pub hour_candles: RwLock<BTreeMap<u64, Candle>>,
//     pub day_candles: RwLock<BTreeMap<u64, Candle>>,
//     pub month_candles: RwLock<BTreeMap<u64, Candle>>
// }

// impl CandleCacheItem{
//     pub fn new() -> Self{
//         CandleCacheItem{
//             minute_candles: RwLock::new(BTreeMap::new()),
//             hour_candles: RwLock::new(BTreeMap::new()),
//             day_candles: RwLock::new(BTreeMap::new()),
//             month_candles: RwLock::new(BTreeMap::new()),
//         }
//     }

//     pub async fn handle_update(&self, candle_type: CandleType, candle: Candle){
//         match candle_type{
//             CandleType::Minute => {
//                 let mut candle_cache = self.minute_candles.write().await;
//                 candle_cache.insert(candle.timestamp, candle);
//             }
//             CandleType::Hour => {
//                 let mut candle_cache = self.hour_candles.write().await;
//                 candle_cache.insert(candle.timestamp, candle);
//             }
//             CandleType::Day => {
//                 let mut candle_cache = self.day_candles.write().await;
//                 candle_cache.insert(candle.timestamp, candle);
//             }
//             CandleType::Month => {
//                 let mut candle_cache = self.month_candles.write().await;
//                 candle_cache.insert(candle.timestamp, candle);
//             }
//         }
//     }

//     async fn exctract_last_minute(&self) -> Option<Candle>{
//         return match self.minute_candles.read().await.iter().next() {
//             Some((_, candle)) => Some(candle.clone()),
//             None => None,
//         }
//     }

//     async fn exctract_last_hour(&self) -> Option<Candle>{
//         return match self.hour_candles.read().await.iter().next() {
//             Some((_, candle)) => Some(candle.clone()),
//             None => None,
//         }
//     }

//     async fn exctract_last_day(&self) -> Option<Candle>{
//         return match self.day_candles.read().await.iter().next() {
//             Some((_, candle)) => Some(candle.clone()),
//             None => None,
//         }
//     }

//     async fn exctract_last_month(&self) -> Option<Candle>{
//         return match self.day_candles.read().await.iter().next() {
//             Some((_, candle)) => Some(candle.clone()),
//             None => None,
//         }
//     }

//     pub async fn exctract_last(&self) -> Option<Candle>{
//         let last_minute = self.exctract_last_minute().await;
        
//         if last_minute.is_some() {
//             self.remove_last(CandleType::Minute, last_minute.clone().unwrap().timestamp).await;
//             return last_minute;
//         }

//         let last_hour = self.exctract_last_hour().await;
        
//         if last_hour.is_some() {
//             self.remove_last(CandleType::Hour, last_hour.clone().unwrap().timestamp).await;
//             return last_hour;
//         }

//         let last_day = self.exctract_last_day().await;

//         if last_day.is_some() {
//             self.remove_last(CandleType::Day, last_day.clone().unwrap().timestamp).await;
//             return last_day;
//         }

//         let last_month = self.exctract_last_month().await;

//         if last_month.is_some() {
//             self.remove_last(CandleType::Month, last_month.clone().unwrap().timestamp).await;
//             return last_month;
//         }

//         return None;
//     }

//     async fn remove_last(&self, candle_type: CandleType, date: u64){
//         match candle_type{
//             CandleType::Minute => {
//                 let mut candle_cache = self.minute_candles.write().await;
//                 candle_cache.remove(&date);
//             }
//             CandleType::Hour => {
//                 let mut candle_cache = self.hour_candles.write().await;
//                 candle_cache.remove(&date);
//             }
//             CandleType::Day => {
//                 let mut candle_cache = self.day_candles.write().await;
//                 candle_cache.remove(&date);
//             }
//             CandleType::Month => {
//                 let mut candle_cache = self.month_candles.write().await;
//                 candle_cache.remove(&date);
//             }
//         }
//     }
// }


// pub struct CandleSaveQueue{
//     pub bids_to_save: BTreeMap<String, CandleCacheItem>,
//     pub asks_to_save: BTreeMap<String, CandleCacheItem>,
// } 

// impl CandleSaveQueue{
//     pub fn new() -> Self{
//         CandleSaveQueue{
//             bids_to_save: BTreeMap::new(),
//             asks_to_save: BTreeMap::new(),
//         }
//     }

//     pub async fn get(&self) -> (Option<Candle>, Option<Candle>){
//         return (self.get_next_item(instrument_id, is_bid), self.get_next_item(instrument_id, is_bid))
//     }

//     pub async fn save_candle(&mut self, instrument_id: &str, is_bid: bool, candle_type: CandleType, candle: Candle){
//         match is_bid{
//             true => {
//                 match self.bids_to_save.get(instrument_id){
//                     Some(cache_item) => cache_item.handle_update(candle_type, candle).await,
//                     None => {
//                         let cache_item = CandleCacheItem::new();
//                         cache_item.handle_update(candle_type, candle).await;
//                         self.bids_to_save.insert(instrument_id.to_string(), cache_item);
//                     },
//                 }},
//             false => {
//                 match self.asks_to_save.get(instrument_id){
//                     Some(cache_item) => cache_item.handle_update(candle_type, candle).await,
//                     None => {
//                         let cache_item = CandleCacheItem::new();
//                         cache_item.handle_update(candle_type, candle).await;
//                         self.bids_to_save.insert(instrument_id.to_string(), cache_item);
//                     },
//                 }
//             }
//         }
//     }

//     async fn get_next_item(&self, is_bid: bool) -> Option<Candle>{
//         match is_bid {
//             true => {

//                 let instrument_id = self.bids_to_save.get(key);

//                 return match self.bids_to_save.get(&instrument_id) {
//                     Some(cache) => cache.exctract_last().await,
//                     None => None,
//                 };
//             },
//             false => {
//                 return match self.asks_to_save.get(&instrument_id) {
//                     Some(cache) => cache.exctract_last().await,
//                     None => None,
//                 };
                
//             },
//         }
//     }
// }