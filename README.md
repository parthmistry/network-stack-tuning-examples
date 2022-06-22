# network-stack-tuning-examples

## Compile and prepare application for running on Linux

### Java application
To compile Java project and copy all dependency jars into a folder
```
cd java-test/
./gradlew clean build copyDependencies
```

### Rust application
To compile Rust project
```
cd rust-test/
cargo build --release
```

## Capacity Test
**Java**  
To run Server application
```
java -cp build/libs/java-test.jar post.parthmistry.nst.capacity.TestServer
```

To run Client application
```
java -cp build/libs/java-test.jar post.parthmistry.nst.capacity.TestClient
```

**Rust**  
To run Server application
```
./target/release/capacity_server
```

To run Client application
```
./target/release/capacity_client
```

## New Connection Test
**Java**  
To run server application
```
java -cp build/libs/java-test.jar post.parthmistry.nst.newconn.TestServer
```
To run client application
```
java -cp build/libs/java-test.jar post.parthmistry.nst.newconn.TestClient
```

**Rust**  
To run server application
```
./target/release/newconn_server
```
To run client application
```
./target/release/newconn_client
```

## Transfer Rate Test
**Java**  
To run server application
```
java -cp build/libs/java-test.jar:dependencies/* post.parthmistry.nst.transferrate.TestSingleServer <server_port>
```
To run client application
```
java -cp build/libs/java-test.jar:dependencies/* post.parthmistry.nst.transferrate.TestSingleClient <server_host> <server_port>
```

**Rust**  
To run server application
```
./target/release/transfer_single_server <server_port>
```
To run client application
```
./target/release/transfer_single_client <server_host> <server_port>
```

## Max Send Test
**Java**  
To run server application
```
java -cp build/libs/java-test.jar:dependencies/* post.parthmistry.nst.maxsend.TestServer <server_port> [conn_wait] [write_wait] [close_wait]
```
To run client application
```
java -cp build/libs/java-test.jar:dependencies/* post.parthmistry.nst.maxsend.TestClient <server_host> <server_port> <num_connections> [close_wait]
```

**Rust**  
To run server application
```
./target/release/maxsend_server <server_port> [conn_wait] [write_wait] [close_wait]
```
To run client application
```
./target/release/maxsend_client <server_host> <server_port> <num_connections> [close_wait]
```

## Check Server TCP Memory
**Java**  
To run application
```
java -cp build/libs/java-test.jar:dependencies/* post.parthmistry.nst.common.CheckServerTCPMemory <server_port>
```

**Rust**  
To run application
```
./target/release/check_server_tcp_memory <server_port>
```
