package post.parthmistry.nst.transferrate;

import post.parthmistry.nst.util.ByteUtil;
import post.parthmistry.nst.util.TimeoutChecker;

import java.io.OutputStream;
import java.net.ServerSocket;
import java.net.Socket;
import java.time.Duration;

public class TestSingleServer {

    private static final byte[] bytes = new byte[1050624];

    public static void main(String[] args) throws Exception {
        int serverPort = Integer.parseInt(args[0]);

        long transferredBytes = 0;

        ServerSocket serverSocket = new ServerSocket(serverPort);
        Socket socket = serverSocket.accept();
        serverSocket.close();

        long startTime = System.currentTimeMillis();
        BitRateLogger bitRateLogger = new BitRateLogger();
        bitRateLogger.start();
        OutputStream outputStream = socket.getOutputStream();
        TimeoutChecker timeoutChecker = new TimeoutChecker(Duration.ofSeconds(60));
        while (!timeoutChecker.isTimeout()) {
            outputStream.write(bytes);
            transferredBytes += bytes.length;
            bitRateLogger.addBytes(bytes.length);
        }
        long endTime = System.currentTimeMillis();
        bitRateLogger.stopBitRateLogger();
        socket.close();

        double transferredMegaBits = ByteUtil.bytesToMegaBits(transferredBytes);
        double durationInSeconds = (endTime - startTime) / 1000.0;

        System.out.println("average bitrate: " + ByteUtil.formatValue(transferredMegaBits / durationInSeconds) + " Mbps");
    }

}
