use crate::model::StatisticsLogger;

fn with_mean_statistics_logger<F: Fn(&dyn StatisticsLogger)>(f:F) -> impl Fn(&dyn StatisticsLogger){
    move |logger: &dyn StatisticsLogger|{
        let avg: f64;
        let stats = logger.get_execution_times();
        let sum : f64 = stats.iter().sum();

        println!("----Mean Statistics----");
        if stats.len() > 0{
            avg = sum / stats.len() as f64;
            println!("avg: {}", avg);
        }

        f(logger);
    }
}

fn with_summary_statistics_logger<F: Fn(&dyn StatisticsLogger)>(f:F) -> impl Fn(&dyn StatisticsLogger){
    move |logger: &dyn StatisticsLogger|{
        let stats = logger.get_execution_times();
        let sum : f64 = stats.iter().sum();
        let min : f64 = *stats.iter().min_by(|x, y| x.total_cmp(y)).unwrap_or(&0.0);
        let max : f64 = *stats.iter().max_by(|x, y| x.total_cmp(y)).unwrap_or(&0.0);

        println!("----Summary Statistics----");
        println!("records: {}", stats.len());
        println!("sum: {}", sum);
        println!("min: {}", min);
        println!("max: {}", max);
        
        f(logger);
    }
}
