use crate::trunc_date;

#[derive(Debug, Clone)]
pub enum CandleType{
    Minute = 0,
    Hour = 1,
    Day = 2,
    Month = 3
}

impl CandleType {
    pub fn parse(int: u8) -> Self{
        match int{
            0 => Self::Minute,
            1 => Self::Hour,
            2 => Self::Day,
            3 => Self::Month,
            _ => panic!("wrong id")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Candle{
    // pub instrument: String,
    // pub candle_type: CandleType,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub timestamp: u64,
}

impl Candle {
    pub fn new(candle_type: CandleType, timestamp: u64, rate: f64) -> Self{
        Self{
            open: rate,
            close: rate,
            high: rate,
            low: rate,
            volume: 0.0,
            timestamp: trunc_date(timestamp, candle_type),
        }
    }

    pub fn update(&mut self, rate: f64){
        self.close = rate;

        if self.high < rate {
            self.high = rate;
        }

        if self.low > rate {
            self.low = rate;
        }
    }
}