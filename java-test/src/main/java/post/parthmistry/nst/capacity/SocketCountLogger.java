package post.parthmistry.nst.capacity;

import java.net.Socket;
import java.util.ArrayList;
import java.util.List;

public class SocketCountLogger extends Thread {

    private final List<Socket> sockets = new ArrayList<>();

    private final String name;

    public SocketCountLogger(String name) {
        this.name = name;
    }

    @Override
    public void run() {
        while (true) {
            try {
                Thread.sleep(2000);
                System.out.println(name + "Sockets: " + sockets.size());
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
    }

    public void addSocket(Socket socket) {
        sockets.add(socket);
    }

}
