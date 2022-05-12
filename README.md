# network-stack-tuning-examples

## Java application
To compile Java project on Linux:
```
cd java-test/
./gradlew clean build
```

### Capacity Test
To run Server application
```
java -cp build/libs/java-test.jar post.parthmistry.nst.capacity.TestServer
```

To run Client application
```
java -cp build/libs/java-test.jar post.parthmistry.nst.capacity.TestClient
```

### New Connection Test
To run server application
```
java -cp build/libs/java-test.jar post.parthmistry.nst.newconn.TestServer
```
To run client application
```
java -cp build/libs/java-test.jar post.parthmistry.nst.newconn.TestClient
```



## Rust application
To compile Rust project on Linux:
```
cd rust-test/
cargo build --release
```

### Capacity Test
To run Server application
```
./target/release/capacity_server
```

To run Client application
```
./target/release/capacity_client
```



### New Connection Test
To run server application
```
./target/release/newconn_server
```
To run client application
```
./target/release/newconn_client
```
