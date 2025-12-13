use crate::{decorators::{WithMeanStatisticsLogger, WithSummaryStatisticsLogger}, model::{ExecutionTimesBaseStatistics, StatisticsLogger}};

mod model;
mod decorators;
mod functional;
fn main() {
    let mut stdout_writer = std::io::stdout();

    let base_stats_logger = ExecutionTimesBaseStatistics::new(vec![10.0, 20.0]);
    base_stats_logger.display_statistics(&mut stdout_writer);

    let with_mean_logger = WithMeanStatisticsLogger::new(base_stats_logger.clone());
    with_mean_logger.display_statistics(&mut stdout_writer);

    let with_summary_logger = WithSummaryStatisticsLogger::new(base_stats_logger.clone());
    with_summary_logger.display_statistics(&mut stdout_writer);

    let double_decorator = WithSummaryStatisticsLogger::new(with_mean_logger.clone());
    double_decorator.display_statistics(&mut stdout_writer);

    //funkcyjne
    let decorated_with_mean = functional::with_mean_statistics_logger(display);
    decorated_with_mean(&base_stats_logger);

    let decorated_with_summarry = functional::with_summary_statistics_logger(display);
    decorated_with_summarry(&base_stats_logger);

    let double = functional::with_summary_statistics_logger(
        functional::with_mean_statistics_logger(display),
    );
    double(&base_stats_logger);

}

fn display(logger: &dyn StatisticsLogger){
    let mut stdout_writer = std::io::stdout();
    logger.display_statistics(&mut stdout_writer);
}
