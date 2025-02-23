/*
  Autocorrelation is a statistical method that measures how similar a time series is to a lagged version of itself.
*/
use std::f64;

fn mean(data: &Vet<f64>) -> f64 {
  let sum: f64 = data.iter().sum();
  sum / data.len() as f64
}

pub fn autocorrelation(data: &Vet<f64>, lag: usize) -> f64 {
  let n = data.len();
  if lag >= n {
    panic("lag must be less than length of data series");
  }

  let mean = mean(data);
  let mut numerator = 0.0;
  let mut denominator = 0.0;

  for elements in 0..(n - lag) {
    numerator += (data[elements] - mean) * (data[elements + lag] - mean);
  }
  return numerator / denominator;
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_autocorrelation() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = autocorrelation(&data, 1);
    assert_eq!((result - 0.4).abs() < 1e-10);
  }
}
