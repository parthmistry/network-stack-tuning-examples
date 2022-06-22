package post.parthmistry.nst.transferrate;

import post.parthmistry.nst.util.ByteUtil;

import java.util.concurrent.atomic.AtomicLong;

public class BitRateLogger extends Thread {

    private final AtomicLong transferredBytes;

    private boolean stopped;

    public BitRateLogger() {
        this.transferredBytes = new AtomicLong(0);
        this.stopped = false;
    }

    @Override
    public void run() {
        while (!stopped) {
            try {
                Thread.sleep(1000);
                synchronized (this) {
                    if (!stopped) {
                        long transferredBytesPerSecond = this.transferredBytes.getAndSet(0);
                        double megaBitsPerSecond = ByteUtil.bytesToMegaBits(transferredBytesPerSecond);
                        System.out.format("bitrate: %5.3f Mbps\n", megaBitsPerSecond);
                    }
                }
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        }
    }

    public void stopBitRateLogger() {
        this.stopped = true;
    }

    public void addBytes(long transferredBytes) {
        this.transferredBytes.addAndGet(transferredBytes);
    }

}
