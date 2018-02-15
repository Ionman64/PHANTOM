use std::env;

///Provides the distance between two points on a graph as represented in cartesian co-ordinates
/// #Example
/// two_dimensions_euclidean_distance((1,1), (2,2)) -> 1.41321: f64
pub fn two_dimensions_euclidean_distance(point_one: (f64, f64), point_two: (f64, f64)) -> f64 {
    let (x1, y1) = point_one;
    let (x2, y2) = point_two;
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

const UP: i16 = 1;
const NONE: i16 = 0;
const DOWN: i16 = -1;

///Detect all the peaks in a Vec<(f64)> and returns the indexes as a Vec<i64, PEAK>
/// #Example
/// let data_set: Vec<i64> = vec![0.0,1.0,0.0]
///
/// let result: Vec<i64, PEAK> = detect_all_peaks(data_set)
///
/// let (x, y) = result.get(0)
///
/// assert_eq!(x, 1)
///
/// assert_eq!(y, PEAK::UP)
pub fn detect_all_peaks(data_set: Vec<f64>) -> Vec<i16> {
    if data_set.len() < 3 {
        panic!("dataset must have more than three elements for peak detection");
    }
    let mut index = 1;
    let array_length = data_set.len();
    let mut downward_trend = false;
    let mut upward_trend = false;
    let mut peak_point = 0;
    let mut return_vector = vec![0; array_length];
    while index < array_length {
        let previous = data_set[index - 1];
        let current = data_set[index];
        if previous < current {
            upward_trend = true;
            if downward_trend {
                return_vector[index-1] = DOWN;
                downward_trend = false;
            }
            peak_point = index;
        } else if previous > current {
            downward_trend = true;
            if upward_trend {
                return_vector[index-1] = UP;
                upward_trend = false;
            }
            peak_point = index;
        }
        index += 1;
    }
    return_vector
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Missing arguments");
        return;
    }
    match args[1].as_str() {
        "--findpeaks" => {
            let ds: Vec<f64> = args.iter().skip(2).map(|x| x.parse().unwrap()).collect();
            let peaks = detect_all_peaks(ds);
            print!("{:?}", peaks);
        }
        _ => {
            println!("Unknown argument {}", args[1]);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_euclidean_distance_1() {
        assert_eq!(two_dimensions_euclidean_distance((1.0, 1.0), (2.0, 2.0)), 1.4142135623730951);
    }
}