pub struct ByteUtil;

impl ByteUtil {

    pub fn transform_bytes(bytes: u64) -> String {
        ByteUtil::transform_bytes_with_f64(bytes, bytes as f64)
    }

    pub fn transform_bytes_with_f64(bytes: u64, double_bytes: f64) -> String {
        format!("{} ({} mb)", bytes, ByteUtil::format_value(double_bytes / (1024.0 * 1024.0)))
    }

    pub fn transform_bytes_f64(bytes: f64) -> String {
        format!("{} ({} mb)", bytes, ByteUtil::format_value(bytes / (1024.0 * 1024.0)))
    }

    pub fn bytes_to_megabits(bytes: u64) -> f64 {
        (bytes as f64 / 1_000_000.0) * 8.0
    }

    pub fn format_value(value: f64) -> String {
        format!("{:.3}", value)
    }

}