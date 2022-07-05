# Rust Engine

This directory contains the code for the Rust engine and Rust example plugins. This is where new 
development will take place going forward.


## Prerequisites

To build the Rust code, you will need to install a recent version of `libzmq` (the ZeroMQ C++ library).
Version 4.1 (or newer) should suffice. 

On Debian/Ubuntu, you can get it by running this command:

```
$ apt-get install libzmq3-dev
```

## Running the example engine
The project currently includes an example with 3 plugins, all written in Rust and
compiled into the main engine, that use inproc sockets for communication and that 
pass flatbuffers messages. It should be enough to build the Rust engine Docker image and run it to see the full example. Using the make commands:

```
$ make build
$ make up-engine
```


## Running the demo
There is also a more simple "demo" which includes three plugins, one in Rust and two in Python,
and that use simple strings for messages. Running the demo currently requires a small code
change to the `main.rs` file -- comment out the line that starts the example engine and recompile.

In order to run the demo, you also need to build the Python image in the `proto` directory. That
directory contains its own Makefile for building the Python image.

Once the Python image is build, use the Makefile in this directory to run the example:

```
$ make up-demo
```

## Development tasks
A collection of reminders for making code changes..

### Updating the flatbuffers messages
The flatbuffers messages schema is defined in the `events.fsb` file. To change the message formats do the following:

1. Edit the `events.fsb` file with your changes.
2. Regenerate the `events_generated.rs` code with the command:

```
$ flatc --rust -o src events.fbs
```
3. Add the following line to the top of the `events_generated.rs` file so that clippy warnings are suppressed:

```
// this line added to keep clippy happy
#![allow(clippy::all)]
```

4. Recompute the message bytes filter and update the `get_event_type_bytes_filter` function
inside the `events.rs` module. This may no need to be done if you haven't added any additional
event types.

Eventually, we won't need to do 4 as the plan is to compute the filters programmatically at run time from a set of existing exemplar messages. 


