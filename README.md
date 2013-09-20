RustyMem
========

Memcached client library for Rust, written in Rust.



## Introduction <a name="Introduction"/>

RustyMem is a robust Memcached client library for Rust programs to connect to Memcached servers.
It supports both the Memcached ASCII protocol and binary protocol for connecting to the Memcached-protocol compatible servers.
It can store cache data in a single server, and supports distributing cache data to a cluster of servers through key sharding.
It sports an easy to use API fully implementing the Memcached protocol to let any Rust program leverage the full feature of a high performaning Memcached system.


## A Hello World Sample <a name="HelloWorld"/>

Here's a quick example to illustrate the usage of RustMem.


    extern mod rustymem;
    
    use rustymem::*;

    let mut rm = rustymem::connect("127.0.0.1");
    rm.set_str("hello", 0, "Hello World");
    println(rm.get_str("hello"));

This imports the public names from the rustymem library.
Creates a new RustMem object with a connection to the Memached server at 127.0.0.1.
Puts the string value "Hello World" with the key "hello" in the server.
Then gets it back by key.

    let mut rm = rustymem::connect("127.0.0.1:11211 127.0.0.1:11212 127.0.0.1:11213");
    rm.set_str("hello", 0, "Hello World");
    println(rm.get_str("hello"));

This creates a RustMem object forming a cluster of three Memcached servers.  A cache object's key
is used to consistently map to a server in the cluster for storing and retrieval.

More examples:

     rm.set_bytes("hello", 60, "Hello World".as_bytes());   // set the byte data with a 60 seconds expiration
     rm.set_as("amount", 60, &10.12);                       // set the amount with the float value type
     rm.set_json("my-vec", 60, &~[10, 20, 30, 40])) );      // set a vector as a JSON object

     rm.get_bytes("hello");                                 // get back the object as a byte array
     rm.get_as::<float>("amount");                          // get back the amount as float type
     rm.get_json("my-vec");                                 // get back the JSON object

     rm.get_bulk_as::<float>(["amount1", "amount2", "amount10"]);  // get multiple objects at one shot




## Dependency <a name="Dependency"/>

The current version of RustMem requires the latest version of Rust at the master branch, 0.8-pre or later.  
It doesn't work with Rust 0.7 since there are substantial changes between 0.7 and 0.8.

RustMem is a pure Rust implementation of a Memcached client and not depending on any native Memcached library.


## Download <a name="Download"/>

Download the source using git

git clone --origin gihub https://github.com/williamw520/rustymem.git

Or download the source tree as a zip file from GitHub.  Then unzip it.


## Build <a name="Build"/>

Run the Makefile to build the library.

    cd rustymem
    make

The output library binary is in the bin directory.  The tests are in there as well.

Run the client_test.  The test assumes there's a Memcached server running at 127.0.0.1:11211, and one running at 127.0.0.1:11212 for the cluster testing.

    bin/client_test

Run the benchmark.  The benchmark assumes there's a Memcached server running at 127.0.0.1:11211

    make bench-client-test

## Sample and Test <a name="Sample"/>

Currently the test file test/client_test.rs has examples of using the RustMem API.

The file client_test.rs shows how a client can use the RustyMem library.  Check the Makefile on how to link in the library.


## Issues <a name="Issues"/>

For now, only IP address can be used as server address.  DNS support is not there yet in Rust.

