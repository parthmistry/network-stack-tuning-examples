package post.parthmistry.nst.util;

import java.util.Arrays;
import java.util.LongSummaryStatistics;

public class StatsUtil {

    public static void printStats(long[] sortedData) {
        LongSummaryStatistics summaryStats = Arrays.stream(sortedData).summaryStatistics();

        System.out.println("min: " + ByteUtil.transformBytes(summaryStats.getMin()));
        System.out.println("max: " + ByteUtil.transformBytes(summaryStats.getMax()));
        System.out.println("average: " + ByteUtil.transformBytes(summaryStats.getAverage()));
        System.out.println("sum: " + ByteUtil.transformBytes(summaryStats.getSum()));

        System.out.println("25th percentile: " + ByteUtil.transformBytes(percentile(sortedData, 25)));
        System.out.println("50th percentile: " + ByteUtil.transformBytes(percentile(sortedData, 50)));
        System.out.println("75th percentile: " + ByteUtil.transformBytes(percentile(sortedData, 75)));
        System.out.println("85th percentile: " + ByteUtil.transformBytes(percentile(sortedData, 85)));
        System.out.println("95th percentile: " + ByteUtil.transformBytes(percentile(sortedData, 95)));
        System.out.println("99th percentile: " + ByteUtil.transformBytes(percentile(sortedData, 99)));
    }

    public static long percentile(long[] sortedData, double percentile) {
        int index = (int) Math.ceil(percentile / 100.0 * sortedData.length);
        return sortedData[index-1];
    }

}
