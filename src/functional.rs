//dekoratory funkcyjne w ruście są straszne, więc aż podopisuję komentarze
use crate::model::StatisticsLogger;
use std::io::{self, Write};

//to jest typ funkcji dekorowanej, jest on przyjmowany i zwracany w każdym dekoratorze
//ta definicja jest tylko dla informacji, bo z jakiegoś powodu nie mogę jej użyć jako return type
//TwT
//musiałem dodać writer, żebym mógł testować co jest wypisywane
type DecoratedFn = dyn Fn(&dyn StatisticsLogger, &mut dyn Write) -> io::Result<()>;

pub fn with_mean_statistics_logger<F>(
    f: F,
) -> impl Fn(&dyn StatisticsLogger, &mut dyn Write) -> io::Result<()>
//where to czytelniejszy sposób zapisania generyków ;3
where
    //'static daje nieskończony lifetime
    F: Fn(&dyn StatisticsLogger, &mut dyn Write) -> io::Result<()> + 'static,
{
    //move zabiera z sobą argumenty podanej funkcji do środka i może ich używać
    move |logger: &dyn StatisticsLogger, writer: &mut dyn Write| {
        let stats = logger.get_execution_times();
        let sum: f64 = stats.iter().sum();

        writeln!(writer, "----Mean Statistics----")?;
        if stats.len() > 0 {
            let avg = sum / stats.len() as f64;
            writeln!(writer, "avg: {:.2}", avg)?;
        }

        //na końcu wywołanie dekorowanej funkcji
        f(logger, writer)
    }
}

//tutaj to samo
pub fn with_summary_statistics_logger<F>(
    f: F,
) -> impl Fn(&dyn StatisticsLogger, &mut dyn Write) -> io::Result<()>
where
    F: Fn(&dyn StatisticsLogger, &mut dyn Write) -> io::Result<()> + 'static,
{
    move |logger: &dyn StatisticsLogger, writer: &mut dyn Write| {
        let stats = logger.get_execution_times();
        let sum: f64 = stats.iter().sum();
        let min: f64 = *stats.iter().min_by(|x, y| x.total_cmp(y)).unwrap_or(&0.0);
        let max: f64 = *stats.iter().max_by(|x, y| x.total_cmp(y)).unwrap_or(&0.0);

        writeln!(writer, "----Summary Statistics----")?;
        writeln!(writer, "records: {}", stats.len())?;
        writeln!(writer, "sum: {:.2}", sum)?;
        writeln!(writer, "min: {:.2}", min)?;
        writeln!(writer, "max: {:.2}", max)?;

        f(logger, writer)
    }
}

//te wszystkie rzeczy tylko po to, żebym mógł je przetestować

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use crate::model::{ExecutionTimesBaseStatistics, StatisticsLogger};

    fn display(logger: &dyn StatisticsLogger, writer: &mut dyn Write) -> std::io::Result<()> {
        logger.display_statistics(writer).unwrap();
        Ok(())
    }

    #[test]
    fn test_with_mean_statistics_logger() {
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
        let mut buffer = Vec::new();
        let with_mean = with_mean_statistics_logger(display);

        with_mean(&exec_times_base_stats.clone(), &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("avg: 15.0"));
        assert!(output.contains("20.0"));
        assert!(output.contains("10.0"));
    }

    #[test]
    fn test_with_mean_statistics_logger_empty() {
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![]);
        let mut buffer = Vec::new();
        let with_mean = with_mean_statistics_logger(display);

        with_mean(&exec_times_base_stats.clone(), &mut buffer).unwrap();
    }

    #[test]
    fn test_with_summary_statistics_logger() {
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
        let mut buffer = Vec::new();
        let with_summary = with_summary_statistics_logger(display);

        with_summary(&exec_times_base_stats.clone(), &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("records: 2"));
        assert!(output.contains("sum: 30.0"));
        assert!(output.contains("min: 10.0"));
        assert!(output.contains("max: 20.0"));
        assert!(output.contains("20.0"));
        assert!(output.contains("10.0"));
    }

    #[test]
    fn test_with_summary_statistics_logger_empty() {
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![]);
        let mut buffer = Vec::new();
        let with_summary = with_summary_statistics_logger(display);

        with_summary(&exec_times_base_stats.clone(), &mut buffer).unwrap();
    }

    #[test]
    fn test_double_logger() {
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
        let mut buffer = Vec::new();
        let double = with_summary_statistics_logger(with_mean_statistics_logger(display));

        double(&exec_times_base_stats, &mut buffer).unwrap();

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
    fn test_double_logger_empty() {
        let exec_times_base_stats = ExecutionTimesBaseStatistics::new(vec![]);
        let mut buffer = Vec::new();
        let double = with_summary_statistics_logger(with_mean_statistics_logger(display));

        double(&exec_times_base_stats, &mut buffer).unwrap();
    }
}
