use std::{io::Write};

pub trait StatisticsLogger {
    fn display_statistics(&self,writer: &mut dyn Write) -> std::io::Result<()>;
    fn get_execution_times(&self) -> Vec<f64>;
}

#[derive(Clone)]
pub struct ExecutionTimesBaseStatistics {
    execution_times: Vec<f64>,
}

impl ExecutionTimesBaseStatistics {
    pub fn new(times: Vec<f64>) -> Self {
        Self {
            execution_times: times,
        }
    }
}

impl StatisticsLogger for ExecutionTimesBaseStatistics {
    fn display_statistics(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, "----Execution times base statistics----")?;
        for record in self.execution_times.clone(){
            writeln!(writer, "{:.2}", record)?;
        }
        Ok(())
    }

    fn get_execution_times(&self) -> Vec<f64> {
        self.execution_times.clone()
    }
}
