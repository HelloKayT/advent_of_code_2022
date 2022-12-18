#[derive(Debug)]
pub struct RangeHelper {
    low: u32,
    high: u32
}
impl RangeHelper {
    pub fn new(range_string: &str) -> Self {
        let mut new_struct = RangeHelper {low: 0, high: 0};
        let (low_str, high_str) = match range_string.split_once("-") {
            Some((low, high)) => (low, high),
            None => panic!("parsing error")
        };

        new_struct.low = low_str.parse().unwrap();
        new_struct.high = high_str.parse().unwrap();

        return new_struct;
    }

    pub fn contains(&self, other: &RangeHelper) -> bool {
        return (self.low <= other.low) && (self.high >= other.high);
    }

    pub fn overlaps(&self, other: &RangeHelper) -> bool {
        let contains = self.contains(other);
        let low_contained = (other.low <= self.low) && (other.high >= self.low);
        let high_contained = (other.low <= self.high) && (other.high >= self.high);
        return contains || low_contained || high_contained;
    }
}