///Provides the distance between two points on a graph as represented in cartesian co-ordinates
/// #Example
/// two_dimensions_euclidean_distance((1,1), (2,2)) -> 1.41321: f64
pub fn two_dimensions_euclidean_distance(point_one: (f64, f64), point_two: (f64, f64)) -> f64 {
    let (x1, y1) = point_one;
    let (x2, y2) = point_two;
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}

pub enum PEAK {
    UP,
    DOWN
}

///Detect all the peaks in a Vec<(f64, f64)> and returns the indexes as a Vec<i64, PEAK>
/// #Example
/// let example_data: Vec<i64, PEAK> = vec![0.0,1.0,0.0]
///
/// let result: Vec<i64, PEAK> = detect_all_peaks(Vec<(f64, f64))
///
/// let (x, y) = result.get(0)
///
/// assert_eq!(x, 1)
///
/// assert_eq!(y, PEAK::UP)
pub fn detect_all_peaks(data_set: Vec<(f64, f64)>) -> Vec<(usize, PEAK)> {
    if data_set.len() < 3 {
        panic!("dataset must have more than three elements for peak detection");
    }
    let mut return_vector:Vec<(usize, PEAK)> = Vec::new();
    let mut index = 1;
    let array_length = data_set.len();
    let mut downward_trend = false;
    let mut upward_trend = false;
    let mut peak_point = 0;
    while index < array_length {
        let (x, previous) = data_set[index-1];
        let (x, current) = data_set[index];
        if previous < current {
            upward_trend = true;
            if downward_trend {
                return_vector.push((peak_point, PEAK::DOWN));
                downward_trend = false;
            }
            peak_point = index;
        }
        if previous > current {
            downward_trend = true;
            if upward_trend {
                return_vector.push((peak_point, PEAK::UP));
                upward_trend = false;
            }
            peak_point = index;
        }
        index += 1;
    }
    return_vector
}