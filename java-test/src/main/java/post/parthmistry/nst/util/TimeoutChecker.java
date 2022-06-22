package post.parthmistry.nst.util;

import java.time.Duration;
import java.time.LocalDateTime;

public class TimeoutChecker {

    private final LocalDateTime timeoutDateTime;

    public TimeoutChecker(Duration duration) {
        this.timeoutDateTime = LocalDateTime.now().plus(duration);
    }

    public boolean isTimeout() {
        return LocalDateTime.now().isAfter(this.timeoutDateTime);
    }

}
