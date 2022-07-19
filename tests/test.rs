#[macro_use]
extern crate approx;

#[cfg(test)]
mod tests {
    use taylor_series::exp;

    #[test]
    fn it_works() {
        let result = exp(1.0, 0.001);
        assert_relative_eq!(2.718, result, epsilon = 0.001);
    }

    #[test]
    fn it_does_not_works() {
        let result = exp(1.0, 0.001);
        assert_relative_ne!(2.7, result, epsilon = 0.001);
    }

    #[test]
    fn test_zero() {
        let result = exp(0.0, 0.001);
        assert_relative_eq!(1.0, result, epsilon = 0.001);
    }

    #[test]
    fn test_negative() {
        let result = exp(-1.0, 0.001);
        assert_relative_eq!(1.0 / 2.718, result, epsilon = 0.001);
    }

    #[test]
    fn test_pos_int() {
        let result = exp(2.0, 0.001);
        assert_relative_eq!(2.718 * 2.718, result, epsilon = 0.1);
    }

    #[test]
    fn test_pos_float() {
        let result = exp(2.5, 0.001);
        assert_relative_eq!(2.718_f64.powf(2.5), result, epsilon = 0.1);
    }

    #[test]
    fn test_delta1() {
        let result = exp(1.0, 1.0);
        assert_relative_eq!(2.718, result, epsilon = 1.0);
        assert_relative_ne!(2.718, result, epsilon = 0.001);
    }

    #[test]
    fn test_delta2() {
        let result_better = exp(1.0, 0.000_001);
        let result_worse = exp(1.0, 0.001);
        let diff_better = (std::f64::consts::E - result_better).abs();
        let diff_worse = (std::f64::consts::E - result_worse).abs();
        assert_eq!(diff_better < diff_worse, true);
    }
}
