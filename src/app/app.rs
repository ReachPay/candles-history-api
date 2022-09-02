use std::sync::Arc;

use rust_extensions::AppStates;

use crate::CandleHistoryCache;

pub struct AppContext{
    pub cache: CandleHistoryCache,
    pub app_states: Arc<AppStates>
}

impl AppContext {
    pub fn new() -> Self{
        Self { cache: CandleHistoryCache::new(), app_states: Arc::new(AppStates::create_initialized()) }
    }

    pub fn new_with_cache(cache: CandleHistoryCache) -> Self{
        Self { cache, app_states: Arc::new(AppStates::create_initialized()) }
    }

}