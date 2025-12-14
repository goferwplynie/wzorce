use std::io::Write;

use crate::model::StatisticsLogger;

#[derive(Clone)]
pub struct WithMeanStatisticsLogger<T: StatisticsLogger> {
    inner: T,
}

impl<T: StatisticsLogger> WithMeanStatisticsLogger<T> {
    pub fn new(inner: T) -> Self {
        WithMeanStatisticsLogger { inner }
    }
}

impl<T: StatisticsLogger> StatisticsLogger for WithMeanStatisticsLogger<T> {
    fn display_statistics(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        let avg: f64;
        let stats = self.inner.get_execution_times();
        let sum: f64 = stats.iter().sum();

        writeln!(writer, "----Mean Statistics----")?;
        if stats.len() > 0 {
            avg = sum / stats.len() as f64;
            writeln!(writer,"avg: {:.2}", avg)?;
        }

        self.inner.display_statistics(writer)?;

        Ok(())
    }

    fn get_execution_times(&self) -> Vec<f64> {
        self.inner.get_execution_times()
    }
}

#[derive(Clone)]
pub struct WithSummaryStatisticsLogger<T: StatisticsLogger> {
    inner: T,
}

impl<T: StatisticsLogger> WithSummaryStatisticsLogger<T> {
    pub fn new(inner: T) -> Self {
        WithSummaryStatisticsLogger { inner }
    }
}

impl<T: StatisticsLogger> StatisticsLogger for WithSummaryStatisticsLogger<T> {
    fn display_statistics(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        let stats = self.inner.get_execution_times();
        let sum: f64 = stats.iter().sum();
        let min: f64 = *stats.iter().min_by(|x, y| x.total_cmp(y)).unwrap_or(&0.0);
        let max: f64 = *stats.iter().max_by(|x, y| x.total_cmp(y)).unwrap_or(&0.0);

        writeln!(writer, "----Summary Statistics----")?;
        writeln!(writer, "records: {}", stats.len())?;
        writeln!(writer, "sum: {:.2}", sum)?;
        writeln!(writer,"min: {:.2}", min)?;
        writeln!(writer,"max: {:.2}", max)?;
        self.inner.display_statistics(writer)?;
        Ok(())
    }

    fn get_execution_times(&self) -> Vec<f64> {
        self.inner.get_execution_times()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::ExecutionTimesBaseStatistics;
    use super::*;

    #[test]
    fn test_with_mean_statistics_logger_display() {
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
        let mean_stats_logger = WithMeanStatisticsLogger::new(exec_times_base_stats.clone());
        let mut buffer = Vec::new();

        mean_stats_logger.display_statistics(&mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();


        assert!(output.contains("avg: 15.0"));
        assert!(output.contains("20.0"));
        assert!(output.contains("10.0"));
    }

    //czy program nie zrobi panica
    #[test]
    fn test_with_mean_statistics_logger_display_empty() {
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![]);
        let mean_stats_logger = WithMeanStatisticsLogger::new(exec_times_base_stats.clone());
        let mut buffer = Vec::new();

        mean_stats_logger.display_statistics(&mut buffer).unwrap();

    }

    #[test]
    fn test_with_mean_statistics_logger_get(){
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
        let mean_stats_logger = WithMeanStatisticsLogger::new(exec_times_base_stats.clone());

        assert!(exec_times_base_stats.get_execution_times() == mean_stats_logger.get_execution_times())
    }

    //summary stats tests
    #[test]
    fn test_with_summary_statistics_logger_display() {
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
        let summary_stats_logger = WithSummaryStatisticsLogger::new(exec_times_base_stats.clone());
        let mut buffer = Vec::new();

        summary_stats_logger.display_statistics(&mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();


        assert!(output.contains("records: 2"));
        assert!(output.contains("sum: 30.0"));
        assert!(output.contains("min: 10.0"));
        assert!(output.contains("max: 20.0"));
        assert!(output.contains("20.0"));
        assert!(output.contains("10.0"));
    }

    //czy program nie zrobi panica
    #[test]
    fn test_with_summary_statistics_logger_display_empty() {
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![]);
        let summary_stats_logger = WithSummaryStatisticsLogger::new(exec_times_base_stats.clone());
        let mut buffer = Vec::new();

        summary_stats_logger.display_statistics(&mut buffer).unwrap();
    }

    #[test]
    fn test_with_summary_statistics_logger_get(){
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
        let summary_stats_logger = WithSummaryStatisticsLogger::new(exec_times_base_stats.clone());

        assert!(exec_times_base_stats.get_execution_times() == summary_stats_logger.get_execution_times())
    }

    #[test]
    fn test_double_logger_display(){
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
        let summary_stats_logger = WithSummaryStatisticsLogger::new(exec_times_base_stats.clone());
        let mean_stats_logger = WithMeanStatisticsLogger::new(summary_stats_logger.clone());

        let mut buffer = Vec::new();

        mean_stats_logger.display_statistics(&mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap();


        assert!(output.contains("avg: 15.0"));
        assert!(output.contains("20.0"));
        assert!(output.contains("10.0"));
        assert!(output.contains("records: 2"));
        assert!(output.contains("sum: 30.0"));
        assert!(output.contains("min: 10.0"));
        assert!(output.contains("max: 20.0"));
        assert!(output.contains("20.0"));
        assert!(output.contains("10.0"));
    }

    #[test]
    fn test_double_logger_display_empty(){
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
        let summary_stats_logger = WithSummaryStatisticsLogger::new(exec_times_base_stats.clone());
        let mean_stats_logger = WithMeanStatisticsLogger::new(summary_stats_logger.clone());

        let mut buffer = Vec::new();

        mean_stats_logger.display_statistics(&mut buffer).unwrap();
    }

    #[test]
    fn test_double_logger_get(){
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
        let summary_stats_logger = WithSummaryStatisticsLogger::new(exec_times_base_stats.clone());
        let mean_stats_logger = WithMeanStatisticsLogger::new(summary_stats_logger.clone());

        assert!(exec_times_base_stats.get_execution_times() == mean_stats_logger.get_execution_times())
    }
}
