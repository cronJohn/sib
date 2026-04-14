use crate::services::ranker::UsageStats;

#[derive(Default, Clone)]
pub struct UsageStatsFixture {
    open_count: u32,
    last_opened: Option<u64>,
}

impl UsageStatsFixture {
    pub fn open_count(mut self, open_count: u32) -> Self {
        self.open_count = open_count;
        self
    }

    pub fn last_opened(mut self, last_opened: Option<u64>) -> Self {
        self.last_opened = last_opened;
        self
    }

    pub fn build(self) -> UsageStats {
        UsageStats {
            open_count: self.open_count,
            last_opened: self.last_opened,
        }
    }
}
