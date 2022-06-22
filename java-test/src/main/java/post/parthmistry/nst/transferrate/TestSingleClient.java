package post.parthmistry.nst.transferrate;

import java.io.InputStream;
import java.net.Socket;

public class TestSingleClient {

    private static final byte[] bytes = new byte[1050624];

    public static void main(String[] args) throws Exception {
        String serverHost = args[0];
        int serverPort = Integer.parseInt(args[1]);

        Socket socket = new Socket(serverHost, serverPort);

        InputStream inputStream = socket.getInputStream();

        while (true) {
            int readBytes = inputStream.read(bytes, 0, bytes.length);
            if (readBytes == -1) {
                break;
            }
        }
        socket.close();
    }

}
