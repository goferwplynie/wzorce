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

        println!("----Mean Statistics----");
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

pub struct WithSummaryStatisticsLogger<T: StatisticsLogger>{
    inner: T
}

impl <T: StatisticsLogger>WithSummaryStatisticsLogger<T> {
    pub fn new(inner: T) -> Self{
        WithSummaryStatisticsLogger { inner }
    }
}

impl <T: StatisticsLogger> StatisticsLogger for WithSummaryStatisticsLogger<T> {
    fn display_statistics(&self) {
        let stats = self.inner.get_execution_times();
        let sum : f64 = stats.iter().sum();
        let min : f64 = *stats.iter().min_by(|x, y| x.total_cmp(y)).unwrap_or(&0.0);
        let max : f64 = *stats.iter().max_by(|x, y| x.total_cmp(y)).unwrap_or(&0.0);

        println!("----Summary Statistics----");
        println!("records: {}", stats.len());
        println!("sum: {}", sum);
        println!("min: {}", min);
        println!("max: {}", max);
        self.inner.display_statistics();
    }

    fn get_execution_times(&self) -> Vec<f64> {
        self.inner.get_execution_times()
    }
}
