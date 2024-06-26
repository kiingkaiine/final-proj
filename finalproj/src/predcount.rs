use std::collections::HashMap;
use crate::FlightRecord;

//Set up regression calculations
pub fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let x_mean = x.iter().sum::<f64>() / x.len() as f64;
    let y_mean = y.iter().sum::<f64>() / y.len() as f64;

    let numerator = x.iter().zip(y.iter()).fold(0.0, |acc, (&xi, &yi)| acc + (xi - x_mean) * (yi - y_mean));
    let denominator = x.iter().fold(0.0, |acc, &xi| acc + (xi - x_mean).powi(2));

    let slope = numerator / denominator;
    let intercept = y_mean - slope * x_mean;

    (slope, intercept)
}

//Use regression to predict following year's passenger volumes each month
pub fn predict_passenger_counts_by_month(records: &[FlightRecord]) -> HashMap<String, u32> {
    let mut passenger_counts_by_month: HashMap<String, u32> = HashMap::new();
    let mut months_seen: Vec<String> = Vec::new();

    //Aggregate passenger counts for each month
    for record in records {
        let activity_period = &record.activity_period;
        let year = &activity_period[..4];
        let month = &activity_period[4..];

        let passenger_count = passenger_counts_by_month.entry(format!("{}-{}", year, month)).or_insert(0);
        *passenger_count += record.passenger_count;

        months_seen.push(format!("{}-{}", year, month));
    }

    //Fit linear regression model
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    for month_str in &months_seen {
        let parts: Vec<&str> = month_str.split('-').collect();
        let year = parts[0].parse::<i32>().unwrap();
        let month = parts[1].parse::<f64>().unwrap();
        let month_num = year as f64 * 12.0 + month;
        x.push(month_num);
        y.push(passenger_counts_by_month[month_str] as f64);
    }

    let (slope, intercept) = linear_regression(&x, &y);

    //Predict passenger counts for the next year
    if let Some(last_month) = months_seen.last() {
        let (last_year, _) = last_month.split_at(4);
        let last_year = last_year.parse::<i32>().unwrap();
        let mut predicted_passenger_counts: HashMap<String, u32> = HashMap::new();
        for month in 1..=12 {
            let month_str = format!("{:02}", month);
            let month_num = last_year * 12 + month as i32;
            let predicted_count = (slope * (month_num as f64) + intercept) as u32;
            predicted_passenger_counts.insert(format!("{}-{}", last_year + 1, month_str), predicted_count);
        }
        predicted_passenger_counts
    } else {
        HashMap::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_linear_regression() {
        //Test 1: Simple linear relationship
        let x1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y1 = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (slope1, intercept1) = linear_regression(&x1, &y1);
        assert_eq!(slope1, 2.0); //Expected slope
        assert_eq!(intercept1, 0.0); //Expected intercept

        //Test 2: Negative slope
        let x2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y2 = vec![10.0, 8.0, 6.0, 4.0, 2.0];
        let (slope2, intercept2) = linear_regression(&x2, &y2);
        assert_eq!(slope2, -2.0); //Expected slope
        assert_eq!(intercept2, 12.0); //Expected intercept
    }

}