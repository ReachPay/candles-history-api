use std::{collections::{BTreeMap, HashMap}};

use tokio::sync::RwLock;

use crate::{CandleType, CandleCacheSaveQueue, Candle, trunc_date, };

pub struct CandleHistoryItem{
    pub candle_type: CandleType,
    pub instrument_id: String,
    pub is_bid: bool,
    // pub save_queue: Arc<RwLock<CandleCacheSaveQueue>>,
    pub cache: RwLock<BTreeMap<u64, Candle>>
}

impl CandleHistoryItem {

    pub fn new(candle_type: CandleType, instrument_id: String, candle: Candle, is_bid: bool) -> Self{
        let mut candles = BTreeMap::new();
        candles.insert(trunc_date(candle.timestamp, candle_type.clone()), candle);
        Self{
            candle_type,
            instrument_id,
            is_bid,
            // save_queue,
            cache: RwLock::new(candles),
        }
    }
    
    pub async fn update(&self, candle: Candle) {
        {
            let mut write_cache = self.cache.write().await;
            write_cache.insert(candle.timestamp.clone(), candle.clone());
        }
        {
            // let mut save_queue = self.save_queue.write().await;
            // save_queue.save_candle(&self.instrument_id, self.is_bid, self.candle_type.clone(), candle).await;
        }
    }

    pub async fn new_rate(&self, date_time: u64, rate: f64){
        let date = trunc_date(date_time, self.candle_type.clone());

        let mut write_cache = self.cache.write().await;
        let candle = write_cache.get_mut(&date);

        match candle {
            Some(candle) => {
                candle.update(rate);
            },
            None => {
                write_cache.insert(date, Candle::new(self.candle_type.clone(), date_time, rate));
            },
        }
    }

    pub async fn get_candles(&self) -> Vec<Candle> {
        let map = self.cache.read().await;
        let values = map.values().cloned().collect::<Vec<Candle>>();

        return values;
    }

    pub async fn get_by_date_range(&self, date_from: u64, date_to: u64) -> Vec<Candle>{
        let map = self.cache.read().await;

        let mut values = Vec::new();

        for (date, candle) in &*map {
            if date > &date_from && date < &date_to{
                values.push(candle.clone());
            }
        }
        return values;
    }

    pub async fn get_first_by_date(&self, request_date: u64) -> Option<Candle>{
        let map = self.cache.read().await;
        let mut value: Option<Candle> = None;

        for (date, candle) in &*map {
            if date > &request_date {
                value = Some(candle.clone());
                break;
            }
        }
        return value;
    }

    pub async fn get_count(&self, size: usize) -> Vec<Candle>{
        let map = self.cache.read().await;
        let mut result: Vec<Candle> = Vec::new();
        let from_index = map.len() - size;

        if from_index <= 0 {
            for (_, candle) in &*map {
                result.push(candle.clone());
            }

            return result;
        }

        map.iter().skip(from_index).for_each(|(_, candle)| {
            result.push(candle.clone());
        });
            
        return result;
    }

    pub async fn init_candle(&self, candle: Candle){
        let mut write_cache = self.cache.write().await;
        write_cache.insert(candle.timestamp.clone(), candle.clone());
    }
}

pub struct CachedCandleModel{
    pub instrument_id: String,
    pub candle: Candle,
    pub candle_type: CandleType,
    pub source: String
} 

impl CachedCandleModel {
    pub fn new(id: String, candle: Candle, candle_type: CandleType, source: String) -> Self{
        Self { instrument_id: id, candle, candle_type, source}
    }
}


pub struct CandleHistoryCache{
    pub save_queue: CandleCacheSaveQueue,
    pub bids: RwLock<HashMap<String, HashMap<u8, CandleHistoryItem>>>,
    pub asks: RwLock<HashMap<String, HashMap<u8, CandleHistoryItem>>>
}

impl CandleHistoryCache {
    pub fn new() -> Self{
        Self {
            save_queue: CandleCacheSaveQueue::new(),
            bids: RwLock::new(HashMap::new()),
            asks: RwLock::new(HashMap::new()),
        }
    }

    pub async fn init(&self, instrument_id: String, is_bid: bool, candle_type: CandleType, candle: Candle){
        match is_bid {
            true => {
                let mut write = self.bids.write().await;
                let target_instrument_candles = write.get_mut(&instrument_id);
                match target_instrument_candles{
                    Some(candles_map) => {

                        let candles_by_type =  candles_map.get(&(candle_type.clone() as u8));

                        match candles_by_type {
                            Some(candle_by_type) => candle_by_type.update(Candle{
                                open: candle.open,
                                close: candle.close,
                                high: candle.high,
                                low: candle.low,
                                volume: candle.volume,
                                timestamp: candle.timestamp,
                            }).await,
                            None => {
                                candles_map.insert(candle_type.clone() as u8, CandleHistoryItem::new(candle_type.clone(), instrument_id.clone(), candle.clone(), is_bid));
                            },
                        }
                    },
                    None => {
                        let mut to_insert = HashMap::new();
                        to_insert.insert(candle_type.clone() as u8, CandleHistoryItem::new(candle_type, instrument_id.clone(), candle, is_bid));
                        write.insert(instrument_id.clone(), to_insert);
                    },
                }
            },
            false => {
                let mut write = self.asks.write().await;
                let target_instrument_candles = write.get_mut(&instrument_id);
                match target_instrument_candles{
                    Some(candles_map) => {

                        let candles_by_type =  candles_map.get(&(candle_type.clone() as u8));

                        match candles_by_type {
                            Some(candle_by_type) => candle_by_type.update(Candle{
                                open: candle.open,
                                close: candle.close,
                                high: candle.high,
                                low: candle.low,
                                volume: candle.volume,
                                timestamp: candle.timestamp,
                            }).await,
                            None => {
                                candles_map.insert(candle_type.clone() as u8, CandleHistoryItem::new(candle_type.clone(), instrument_id.clone(), candle.clone(), is_bid));
                            },
                        }
                    },
                    None => {
                        let mut to_insert = HashMap::new();
                        to_insert.insert(candle_type.clone() as u8, CandleHistoryItem::new(candle_type, instrument_id.clone(), candle, is_bid));
                        write.insert(instrument_id.clone(), to_insert);
                    },
                }
            },
        }
    }

    pub async fn get_in_date_range(&self, instrument_id: String, candle_type: CandleType, is_bid: bool,
        from: u64, to: u64) -> Vec<Candle>{

            let mut candles: Vec<Candle> = Vec::new();

            match is_bid{
                true => {
                    {
                        let bids = self.bids.read().await;
                        let history = bids.get(&instrument_id);
    
                        match history{
                            Some(candle_types) => {
                                let history_item = candle_types.get(&(candle_type as u8));
                                match history_item {
                                    Some(history_item) => {
                                        let cache_candles = history_item.get_by_date_range(from, to).await;
                                        candles = cache_candles;
                                    },
                                    None => {},
                                }
                            },
                            None => {},
                        }
                    }
                },
                false => {
                    let asks = self.asks.read().await;
                    let history = asks.get(&instrument_id);

                    match history{
                        Some(candle_types) => {
                            let history_item = candle_types.get(&(candle_type as u8));
                            match history_item {
                                Some(history_item) => {
                                    let cache_candles = history_item.get_by_date_range(from, to).await;
                                    candles = cache_candles;
                                },
                                None => {},
                            }
                        },
                        None => {},
                    }
                }
            }

            
        return candles;
    }

    // pub async fn get_all_from_cache(&self, is_bids: bool) -> Vec<CachedCandleModel>{
    //     let target = match is_bids{
    //         true => self.asks.read().await,
    //         false => self.bids.read().await,
    //     };

    //     let result: Vec<CachedCandleModel> = Vec::new();

    //     for (instrument, inner) in &*target{
    //         for (candle_type, item) in &*inner{
    //             result.push(CachedCandleModel::new(instrument.clone(), candle, candle_type, source))
    //         }
    //     }

    //     return result;
    // }

    
}