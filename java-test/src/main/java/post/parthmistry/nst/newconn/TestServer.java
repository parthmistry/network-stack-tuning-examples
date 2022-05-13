package post.parthmistry.nst.newconn;

import java.io.IOException;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class TestServer {

    public static void main(String[] args) throws Exception {
        SocketStatLogger socketStatLogger = new SocketStatLogger();
        socketStatLogger.start();
        ExecutorService executor = Executors.newFixedThreadPool(4);
        try (ServerSocket serverSocket = new ServerSocket(9393)) {
            while (true) {
                try {
                    Socket socket = serverSocket.accept();
                    socketStatLogger.addSuccessCount();
                    executor.submit(() -> {
                        try {
                            socket.getInputStream().read();
                            socket.close();
                        } catch (IOException e) {
                            throw new RuntimeException(e);
                        }
                    });
                } catch (Exception e) {
                    socketStatLogger.addErrorCount(e.getMessage());
                }
            }
        }
    }

}
