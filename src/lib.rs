//! `bucketize` is a crate for slotting numerical values into buckets

#[derive(Clone, Debug, PartialEq)]
pub struct Bucketizer {
    buckets: Vec<Bucket>
}

type Bucket = (Option<f64>, Option<f64>, f64);

impl Bucketizer {
    pub fn new() -> Self {
        Bucketizer {
            buckets: Vec::new(),
        }
    }

    pub fn bucket(self, min: Option<f64>, max: Option<f64>, value: f64) -> Self {
        let mut new = self;
        new.buckets.push((min, max, value));
        new
    }

    pub fn bucketize(&self, input: f64) -> Option<f64> {
        for bucket in &self.buckets {
            match *bucket {
                (None, None, val) => return Some(val),
                (Some(min), None, val) => {
                    if input >= min { return Some(val); }
                }
                (None, Some(max), val) => {
                    if input < max { return Some(val); }
                }
                (Some(min), Some(max), val) => {
                    if input >= min && input < max { return Some(val); }
                }
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::Bucketizer;

    #[test]
    fn single_bucket_middle_values() {
        let b = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(b.bucketize(0.1), Some(0.5));
        assert_eq!(b.bucketize(999.999), None);
    }

    #[test]
    fn single_bucket_end_values() {
        let b = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(b.bucketize(0.0), Some(0.5));
        assert_eq!(b.bucketize(1.0), None);
    }

    #[test]
    fn multiple_buckets_closed_ends() {
        let b = Bucketizer::new()
            .bucket(Some(-1.0), Some(0.0), -0.5)
            .bucket(Some(0.0), Some(1.0), 0.5);
        assert_eq!(b.bucketize(0.0), Some(0.5));
        assert_eq!(b.bucketize(-0.7), Some(-0.5));
        assert_eq!(b.bucketize(999.99), None);
    }

    #[test]
    fn multiple_buckets_opened_ends() {
        let b = Bucketizer::new()
            .bucket(Some(0.0), Some(1.0), 0.5)
            .bucket(Some(1.0), None, 1.5);
        assert_eq!(b.bucketize(0.0), Some(0.5));
        assert_eq!(b.bucketize(-0.7), None);
        assert_eq!(b.bucketize(999.99), Some(1.5));
    }
}
