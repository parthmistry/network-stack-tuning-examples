package post.parthmistry.nst.common;

import post.parthmistry.nst.util.ByteUtil;
import post.parthmistry.nst.util.StatsUtil;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

class SocketMemory {

    private final int remotePort;

    private final int rmemAlloc;

    private final int rcvBuf;

    private final int wmemAlloc;

    private final int sndBuf;

    private final int wmemQueued;

    public SocketMemory(int remotePort, int rmemAlloc, int rcvBuf, int wmemAlloc, int sndBuf, int wmemQueued) {
        this.remotePort = remotePort;
        this.rmemAlloc = rmemAlloc;
        this.rcvBuf = rcvBuf;
        this.wmemAlloc = wmemAlloc;
        this.sndBuf = sndBuf;
        this.wmemQueued = wmemQueued;
    }

    public int getRemotePort() {
        return remotePort;
    }

    public int getRmemAlloc() {
        return rmemAlloc;
    }

    public int getRcvBuf() {
        return rcvBuf;
    }

    public int getWmemAlloc() {
        return wmemAlloc;
    }

    public int getSndBuf() {
        return sndBuf;
    }

    public int getWmemQueued() {
        return wmemQueued;
    }

    @Override
    public String toString() {
        return "remote_port=" + remotePort +
                ", rmem_alloc=" + ByteUtil.transformBytes(rmemAlloc) +
                ", rcv_buf=" + ByteUtil.transformBytes(rcvBuf) +
                ", wmem_alloc=" + ByteUtil.transformBytes(wmemAlloc) +
                ", snd_buf=" + ByteUtil.transformBytes(sndBuf) +
                ", wmem_queued=" + ByteUtil.transformBytes(wmemQueued);
    }
}

public class CheckServerTCPMemory {

    public static void main(String[] args) throws Exception {
        int serverPort = Integer.parseInt(args[0]);

        Process p = Runtime.getRuntime().exec("ss -tm");

        BufferedReader reader = new BufferedReader(new InputStreamReader(p.getInputStream()));

        Pattern statsLinePattern = Pattern.compile("r([\\d]+),rb([\\d]+),t([\\d]+),tb([\\d]+),f([\\d]+),w([\\d]+),o([\\d]+),bl([\\d]+),d([\\d]+)");

        Pattern socketLinePattern = Pattern.compile(".*:" + serverPort + ".*:([\\d]+)");

        String line = null;
        List<SocketMemory> socketMemories = new ArrayList<>();
        while ((line = reader.readLine()) != null) {
            Matcher socketLineMatcher = socketLinePattern.matcher(line);
            if (socketLineMatcher.find()) {
                String remotePort = socketLineMatcher.group(1);
                String statsLine = reader.readLine();
                if (statsLine != null) {
                    Matcher statsLineMatcher = statsLinePattern.matcher(statsLine);
                    if (statsLineMatcher.find()) {
                        SocketMemory socketMemory = new SocketMemory(
                                Integer.parseInt(remotePort),
                                Integer.parseInt(statsLineMatcher.group(1)),
                                Integer.parseInt(statsLineMatcher.group(2)),
                                Integer.parseInt(statsLineMatcher.group(3)),
                                Integer.parseInt(statsLineMatcher.group(4)),
                                Integer.parseInt(statsLineMatcher.group(6))
                        );
                        socketMemories.add(socketMemory);
                    } else {
                        throw new RuntimeException("unexpected error occurred");
                    }
                }
            }
        }

        socketMemories.sort(Comparator.comparingInt(SocketMemory::getWmemQueued));
        for (int i = 1; i <= socketMemories.size(); i++) {
            System.out.format("%7d : %s\n", i, socketMemories.get(i - 1));
        }
        StatsUtil.printStats(socketMemories.stream().mapToLong(SocketMemory::getWmemQueued).toArray());
    }

}
