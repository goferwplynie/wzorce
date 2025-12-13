pub trait StatisticsLogger{
    fn display_statistics(&self);
    fn get_execution_times(&self) -> Vec<f64>;
}

pub struct ExecutionTimesBaseStatistics{
    execution_times: Vec<f64>
}

impl ExecutionTimesBaseStatistics {
    pub fn new(times: Vec<f64>) -> Self {
        Self { execution_times: times }
    }
}

impl StatisticsLogger for ExecutionTimesBaseStatistics {
    fn display_statistics(&self) {
        println!("----Execution times base statistics----");
        println!("{:#?}", self.execution_times)
    }

    fn get_execution_times(&self) -> Vec<f64> {
        self.execution_times.clone()
    }
}
