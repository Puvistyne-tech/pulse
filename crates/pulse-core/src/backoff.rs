#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackoffPolicy {
    pub initial_ms: u32,
    pub max_ms: u32,
}

impl Default for BackoffPolicy {
    fn default() -> Self {
        Self {
            initial_ms: 500,
            max_ms: 30_000,
        }
    }
}

impl BackoffPolicy {
    pub fn delay_ms(&self, attempt: u32) -> u32 {
        self.initial_ms
            .saturating_mul(2u32.saturating_pow(attempt))
            .min(self.max_ms)
    }
}
