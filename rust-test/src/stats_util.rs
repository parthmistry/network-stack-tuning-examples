use crate::byte_util::ByteUtil;

pub struct StatsUtil;

impl StatsUtil {

    pub fn print_stats(sorted_data: &[u64]) {
        let min: u64 = *sorted_data.iter().min().unwrap();
        let max: u64 = *sorted_data.iter().max().unwrap();
        let sum: u64 = sorted_data.iter().sum::<u64>();
        let average: f64 = sum as f64 / sorted_data.len() as f64;

        println!("min: {}", ByteUtil::transform_bytes(min));
        println!("max: {}", ByteUtil::transform_bytes(max));
        println!("average: {}", ByteUtil::transform_bytes_f64(average));
        println!("sum: {}", ByteUtil::transform_bytes(sum));

        println!("25th percentile: {}", ByteUtil::transform_bytes(StatsUtil::percentile(sorted_data, 25)));
        println!("50th percentile: {}", ByteUtil::transform_bytes(StatsUtil::percentile(sorted_data, 50)));
        println!("75th percentile: {}", ByteUtil::transform_bytes(StatsUtil::percentile(sorted_data, 75)));
        println!("85th percentile: {}", ByteUtil::transform_bytes(StatsUtil::percentile(sorted_data, 85)));
        println!("95th percentile: {}", ByteUtil::transform_bytes(StatsUtil::percentile(sorted_data, 95)));
        println!("99th percentile: {}", ByteUtil::transform_bytes(StatsUtil::percentile(sorted_data, 99)));

    }

    fn percentile(sorted_data: &[u64], percentile: u64) -> u64 {
        let index = (percentile as f64 / 100.0 * sorted_data.len() as f64).ceil() as usize;
        return sorted_data[index-1];
    }

}