package post.parthmistry.nst.newconn;

import java.net.Socket;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class TestClient {

    public static void main(String[] args) throws Exception {
        SocketStatLogger socketStatLogger = new SocketStatLogger();
        socketStatLogger.start();
        final int parallelCount = 8;
        ExecutorService executor = Executors.newFixedThreadPool(parallelCount);
        for (int i = 0; i < parallelCount; i++) {
            executor.submit(() -> {
                while (true) {
                    try {
                        Socket socket = new Socket("pmload2.southeastasia.cloudapp.azure.com", 9393);
                        socketStatLogger.addSuccessCount();
                        socket.close();
                    } catch (Exception e) {
                        socketStatLogger.addErrorCount(e.getMessage());
                    }
                }
            });
        }
        socketStatLogger.join();
    }

}
