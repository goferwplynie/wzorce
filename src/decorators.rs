use crate::model::StatisticsLogger;

pub struct WithMeanStatisticsLoggger<T: StatisticsLogger>{
    inner: T
}

impl <T: StatisticsLogger>WithMeanStatisticsLoggger<T> {
    pub fn new(inner: T) -> Self{
        WithMeanStatisticsLoggger { inner }
    }
}

impl <T: StatisticsLogger> StatisticsLogger for WithMeanStatisticsLoggger<T> {
    fn display_statistics(&self) {
        let avg: f64;
        let stats = self.inner.get_execution_times();
        let sum : f64 = stats.iter().sum();

        if stats.len() > 0{
            avg = sum / stats.len() as f64;
            println!("avg: {}", avg);
        }

        self.inner.display_statistics();
    }

    fn get_execution_times(&self) -> Vec<f64> {
        self.inner.get_execution_times()
    }
}
