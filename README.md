RustyMem
========

Memcached client library in Rust.



# Introduction <a name="Introduction"/>

RustyMem provides a robust Memcached client library for Rust programs to connect to Memcached servers.
It supports both the ASCII Memcached protocol and binary Memcached protocol for connecting to the newer versions of the servers.
It supports storing cache data in a single server and distributing cache data to a cluster of servers with key space partition.
It sports an easy to use API to let any Rust programs to leverage the high performaning Memcached system.


## A Hello World Sample <a name="HelloWorld"/>

Here's a quick example to illustrate the usage of RustMem.


    extern mod rustymem;
    
    use rustymem::*;

    let mut rm = rustymsem::new_rusty_mem("127.0.0.1");
    rm.set_str("hello", 0, "Hello World");
    println(rm.get_str("hello"));

This imports the public names from the rustymem library.
Creates a new RustMem object with a connection to the Memached server at 127.0.0.1, at default port 11211.
Puts the string value "Hello World" at key "hello" in the Memached server.
Then gets it back.

    let mut rm = rustymsem::new_rusty_mem("127.0.0.1:11211 127.0.0.1:11212 127.0.0.1:11213");
    rm.set_str("hello", 0, "Hello World");
    println(rm.get_str("hello"));

This Creates a RustMem object connecting to a cluster of Memcached servers.  A cache object's key
is used to distribute it to a server in the cluster for storing and retrieval.


## Dependency <a name="Dependency"/>

The current version of RustMem requires the latest version of Rust at master, 0.8-pre or later.


## Buile <a name="Build"/>

Run the Makefile to build the library.

    cd rustymem
    make

The output library binary is in the bin directory.

## Sample and Test <a name="Sample"/>

Currently the test file test/client_test.rs has examples of using the RustMem API.


## Issues <a name="Issues"/>

For now, only IP address can be used as server address.  DNS support is not yet in Rust.

The ASCII Memcached protocol has been mostly implemeneted.  Only a portion of the binary protocol has been built.

