use std::f64;

/// Conducts an A/B test on two given proportions and outputs the winner if any.
///
/// The function performs a hypothesis test of the null hypothesis that the two 
/// proportions are equal. If the null could be rejected at the 5% significance 
/// level, the function will return the version with the higher conversion rate, 
/// along with its 95% confidence interval for difference of proportions.
///
/// # Arguments
///
/// * `p1` - The conversion rate of version A.
/// * `n1` - The number of samples exposed to version A.
/// * `p2` - The conversion rate of version B.
/// * `n2` - The number of samples exposed to version B.
///
/// # Example
///
/// ```
/// let p1 = 0.20;
/// let n1 = 1000;
/// let p2 = 0.33;
/// let n2 = 800;
/// let result = ab_conversion_test(p1, n1, p2, n2);
/// ```
fn ab_conversion_test(p1: f64, n1: usize, p2: f64, n2: usize) -> Result<(f64, f64, f64), &'static str> {
    if n1 < 5 || n2 < 5 {
        return Err("Insufficient sample size.");
    }
    let p = (p1 * (n1 as f64) + p2 * (n2 as f64)) / ((n1 + n2) as f64);
    let numerator = (p1 - p2).abs();
    let denominator = (p * (1.0 - p) * ((1.0 / (n1 as f64)) + (1.0 / (n2 as f64)))).sqrt();
    let z = numerator / denominator;

    // 95% confidence interval for difference of proportions
    let moe = 1.96 * denominator;  // margin of error
    let lo = numerator - moe;
    let hi = numerator + moe;

    if z > 1.96 {
        Ok((if p1 > p2 { n1 } else { n2 } as f64, lo, hi))
    } else {
        Err("No statistically significant difference was found.")
    }
}

fn main() {
    let n1 = 1000;
    let x1 = 200;
    let n2 = 800;
    let x2 = 560;
    let p1 = x1 as f64 / n1 as f64;
    let p2 = x2 as f64 / n2 as f64;
    
    match ab_conversion_test(p1, n1, p2, n2) {
        Ok((winner, lo, hi)) if winner as usize == n1 => println!("Version A is the winner!\nThe increase in conversion rates is likely between {:.2}% and {:.2}%.", lo*100., hi*100.),
        Ok((winner, lo, hi)) if winner as usize == n2 => println!("Version B is the winner!\nThe increase in conversion rates is likely between {:.2}% and {:.2}%.", lo*100., hi*100.),
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}