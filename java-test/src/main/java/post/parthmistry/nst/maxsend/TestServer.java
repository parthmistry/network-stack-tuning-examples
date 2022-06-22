package post.parthmistry.nst.maxsend;

import io.netty.bootstrap.ServerBootstrap;
import io.netty.buffer.ByteBuf;
import io.netty.buffer.ByteBufAllocator;
import io.netty.channel.*;
import io.netty.channel.nio.NioEventLoopGroup;
import io.netty.channel.socket.SocketChannel;
import io.netty.channel.socket.nio.NioServerSocketChannel;
import io.netty.util.concurrent.Future;
import io.netty.util.concurrent.GenericFutureListener;
import post.parthmistry.nst.util.StatsUtil;

import java.time.Duration;
import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;

class MaxSendWriteListener implements GenericFutureListener<Future<? super Void>> {

    ChannelHandlerContext ctx;

    MaxSendHandler maxSendHandler;

    public MaxSendWriteListener(ChannelHandlerContext ctx, MaxSendHandler maxSendHandler) {
        this.ctx = ctx;
        this.maxSendHandler = maxSendHandler;
    }

    @Override
    public void operationComplete(Future<? super Void> future) {
        if (future.isSuccess()) {
            maxSendHandler.sentBytes += 64000;
            if (ctx.channel().isWritable()) {
                ctx.writeAndFlush(MaxSendHandler.buffer.duplicate().retain()).addListener(this);
            }
        } else {
            if (!future.cause().getClass().getName().contains("StacklessClosedChannelException")) {
                future.cause().printStackTrace();
            }
        }
    }

}

class MaxSendHandler extends ChannelInboundHandlerAdapter {

    public static ByteBuf buffer = ByteBufAllocator.DEFAULT.buffer(64000);

    public long sentBytes = 0;

    private ChannelHandlerContext ctx;

    static {
        buffer.writeBytes(new byte[64000]);
    }

    @Override
    public void channelActive(ChannelHandlerContext ctx) {
        this.ctx = ctx;
    }

    @Override
    public void channelRead(ChannelHandlerContext ctx, Object msg) {
        ((ByteBuf) msg).release();
    }

    @Override
    public void channelWritabilityChanged(ChannelHandlerContext ctx) {
        if (ctx.channel().isWritable()) {
            this.startWriting();
        }
    }

    @Override
    public void exceptionCaught(ChannelHandlerContext ctx, Throwable cause) {
        cause.printStackTrace();
        ctx.close();
    }

    public void startWriting() {
        MaxSendWriteListener listener = new MaxSendWriteListener(ctx, this);
        synchronized (MaxSendHandler.class) {
            ctx.writeAndFlush(buffer.duplicate().retain()).addListener(listener);
        }
    }

    public long getSentBytes() {
        return sentBytes;
    }

    public void close() {
        this.ctx.close().syncUninterruptibly();
    }

}

public class TestServer {

    public static void main(String[] args) throws Exception {
        int serverPort = Integer.parseInt(args[0]);
        int connectionWaitDuration = 15;
        int writeWaitDuration = 15;
        int closeWaitDuration = 30;

        if (args.length >= 2) {
            connectionWaitDuration = Integer.parseInt(args[1]);
        }

        if (args.length >= 3) {
            writeWaitDuration = Integer.parseInt(args[2]);
        }

        if (args.length >= 4) {
            closeWaitDuration = Integer.parseInt(args[3]);
        }

        List<MaxSendHandler> maxSendHandlers = new CopyOnWriteArrayList<>();

        EventLoopGroup bossGroup = new NioEventLoopGroup(1);
        EventLoopGroup workerGroup = new NioEventLoopGroup();

        ServerBootstrap b = new ServerBootstrap();
        b.group(bossGroup, workerGroup)
                .channel(NioServerSocketChannel.class)
                .childHandler(new ChannelInitializer<SocketChannel>() {
                    @Override
                    protected void initChannel(SocketChannel ch) {
                        MaxSendHandler maxSendHandler = new MaxSendHandler();
                        ch.pipeline().addLast(maxSendHandler);
                        maxSendHandlers.add(maxSendHandler);
                    }
                });

        ChannelFuture f = b.bind(serverPort).sync();
        System.out.println("server started");

        Thread.sleep(Duration.ofSeconds(connectionWaitDuration).toMillis());

        System.out.println("start writing to connections");
        maxSendHandlers.forEach(MaxSendHandler::startWriting);

        Thread.sleep(Duration.ofSeconds(writeWaitDuration).toMillis());

        long[] sortedSentBytes = maxSendHandlers.stream().mapToLong(MaxSendHandler::getSentBytes).sorted().toArray();

        StatsUtil.printStats(sortedSentBytes);

        Thread.sleep(Duration.ofSeconds(closeWaitDuration).toMillis());

        System.out.println("start closing connections");
        maxSendHandlers.forEach(MaxSendHandler::close);

        bossGroup.shutdownGracefully();
        workerGroup.shutdownGracefully();
        f.channel().close().sync();
    }

}
