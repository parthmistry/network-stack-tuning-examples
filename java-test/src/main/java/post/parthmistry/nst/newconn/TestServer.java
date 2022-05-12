package post.parthmistry.nst.newconn;

import java.io.IOException;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

public class TestServer {

    public static void main(String[] args) throws Exception {
        SocketStatLogger socketStatLogger = new SocketStatLogger();
        socketStatLogger.start();
        ScheduledExecutorService executor = Executors.newScheduledThreadPool(4);
        try (ServerSocket serverSocket = new ServerSocket(9393)) {
            while (true) {
                try {
                    Socket socket = serverSocket.accept();
                    socketStatLogger.addSuccessCount();
                    executor.schedule(() -> {
                        try {
                            socket.close();
                        } catch (IOException e) {
                            throw new RuntimeException(e);
                        }
                    }, 50, TimeUnit.MILLISECONDS);
                } catch (Exception e) {
                    socketStatLogger.addErrorCount(e.getMessage());
                }
            }
        }
    }

}
