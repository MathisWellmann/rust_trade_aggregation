use crate::modules::FeatureModule;
use crate::Trade;

/// Measures the velocity of candle creation based on the formula:
/// 1.0 / t  , where t is measured in minutes
/// The higher the velocity the faster the candle has been created
#[derive(Debug, Default)]
pub struct ModuleTimeVelocity {
    init_time: i64,
    last_time: i64,
}

impl FeatureModule for ModuleTimeVelocity {
    fn name(&self) -> &str {
        "time_velocity"
    }

    fn value(&self) -> f64 {
        let mut elapsed_s: f64 = (self.last_time - self.init_time) as f64 / 1000.0;
        if elapsed_s < 1.0 {
            // cap elapsed_s to avoid time_velocity being infinite
            elapsed_s = 1.0;
        }
        1.0 / elapsed_s
    }

    fn update(&mut self, trade: &Trade, init: bool) {
        if init {
            self.init_time = trade.timestamp;
        }
        self.last_time = trade.timestamp;
    }
}

