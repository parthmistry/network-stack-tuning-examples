package post.parthmistry.nst.capacity;

import java.net.ServerSocket;
import java.net.Socket;

public class TestServer {

    public static void main(String[] args) throws Exception {
        SocketCountLogger socketCountLogger = new SocketCountLogger("accepted");
        socketCountLogger.start();
        ServerSocket serverSocket = new ServerSocket(9393);
        while (true) {
            try {
                Socket socket = serverSocket.accept();
                socketCountLogger.addSocket(socket);
            } catch (Exception e) {
                e.printStackTrace();
                break;
            }
        }
        serverSocket.close();
        socketCountLogger.join();
    }

}
