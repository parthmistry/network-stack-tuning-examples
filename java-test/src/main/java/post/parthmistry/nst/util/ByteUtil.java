package post.parthmistry.nst.util;

import java.text.DecimalFormat;

public class ByteUtil {

    private static final DecimalFormat decimalFormat = new DecimalFormat();

    static {
        decimalFormat.setMaximumFractionDigits(3);
        decimalFormat.setGroupingUsed(false);
    }

    public static String transformBytes(long bytes) {
        return transformBytes(bytes, (double) bytes);
    }

    public static String transformBytes(double bytes) {

        return bytes + " (" + formatValue((bytes / (1024 * 1024))) + " mb)";
    }

    public static String transformBytes(long bytes, double doubleBytes) {
        return bytes + " (" + formatValue((doubleBytes / (1024 * 1024))) + " mb)";
    }

    public static double bytesToMegaBits(long bytes) {
        return (((double) bytes) / 1_000_000) * 8;
    }

    public static String formatValue(double value) {
        return decimalFormat.format(value);
    }

}
