use crate::mt5_apiserver::IMTTradeSink;

#[derive(Debug)]
pub struct TradeSink {
    mt_trade_sink: *mut IMTTradeSink
}

impl TradeSink {
    pub fn new(mt_trade_sink: *mut IMTTradeSink) -> TradeSink {
        TradeSink {
            mt_trade_sink
        }
    }
}
