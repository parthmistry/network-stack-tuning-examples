package post.parthmistry.nst.capacity;

import java.net.Socket;

public class TestClient {

    public static void main(String[] args) throws Exception {
        SocketCountLogger socketCountLogger = new SocketCountLogger("connected");
        socketCountLogger.start();
        while (true) {
            try {
                Socket socket = new Socket("pmload2.southeastasia.cloudapp.azure.com", 9393);
                socketCountLogger.addSocket(socket);
            } catch (Exception e) {
                e.printStackTrace();
                break;
            }
        }
        socketCountLogger.join();
    }

}
