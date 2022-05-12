package post.parthmistry.nst.newconn;

import java.util.HashSet;
import java.util.Set;

public class SocketStatLogger extends Thread {

    private long successCount;

    private long errorCount;

    private final Set<String> errorMessages;

    public SocketStatLogger() {
        this.successCount = 0;
        this.errorCount = 0;
        errorMessages = new HashSet<>();
    }

    @Override
    public void run() {
        while (true) {
            try {
                Thread.sleep(2000);
                synchronized (this) {
                    System.out.println("success: " + successCount + ", error: " + errorCount);
                    errorMessages.forEach(System.out::println);
                    successCount = 0;
                    errorCount = 0;
                    errorMessages.clear();
                }

            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
    }

    public synchronized void addSuccessCount() {
        successCount++;
    }

    public synchronized void addErrorCount(String errorMessage) {
        errorCount++;
        errorMessages.add(errorMessage);
    }

}
