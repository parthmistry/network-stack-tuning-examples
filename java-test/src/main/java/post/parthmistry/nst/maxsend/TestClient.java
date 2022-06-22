package post.parthmistry.nst.maxsend;

import io.netty.bootstrap.Bootstrap;
import io.netty.channel.*;
import io.netty.channel.nio.NioEventLoopGroup;
import io.netty.channel.socket.SocketChannel;
import io.netty.channel.socket.nio.NioSocketChannel;

import java.time.Duration;
import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;

class ClientHandler extends ChannelInboundHandlerAdapter {

    private ChannelHandlerContext ctx;

    @Override
    public void channelActive(ChannelHandlerContext ctx) {
        TestClient.addConnectedCount();
        this.ctx = ctx;
    }

    public void close() {
        this.ctx.close().syncUninterruptibly();
    }

}

public class TestClient {

    private static long connectedCount = 0;

    public static void main(String[] args) throws Exception {
        String serverHost = args[0];
        int serverPort = Integer.parseInt(args[1]);
        int parallelCount = Integer.parseInt(args[2]);
        int closeWaitDuration = 90;

        if (args.length >= 4) {
            closeWaitDuration = Integer.parseInt(args[3]);
        }

        EventLoopGroup workerGroup = new NioEventLoopGroup();
        List<ClientHandler> clientHandlers = new CopyOnWriteArrayList<>();
        for (int i = 0; i < parallelCount; i++) {
            Bootstrap b = new Bootstrap().group(workerGroup)
                    .channel(NioSocketChannel.class)
                    .option(ChannelOption.AUTO_READ, false)
                    .handler(new ChannelInitializer<SocketChannel>() {
                        @Override
                        protected void initChannel(SocketChannel ch) {
                            ClientHandler clientHandler = new ClientHandler();
                            ch.pipeline().addLast(clientHandler);
                            clientHandlers.add(clientHandler);
                        }
                    });
            b.connect(serverHost, serverPort);
        }

        Thread.sleep(Duration.ofSeconds(closeWaitDuration).toMillis());

        System.out.println("start closing connections");
        clientHandlers.forEach(ClientHandler::close);

        workerGroup.shutdownGracefully();
    }

    public static synchronized void addConnectedCount() {
        System.out.println("connected sockets: " + (++connectedCount));
    }

}
