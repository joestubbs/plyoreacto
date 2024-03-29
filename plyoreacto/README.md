# Rust Engine and Plugins

This directory contains the code for the Rust engine and Rust example plugins. This is where new 
development will take place going forward. It also contains the `pyobserver` directory with code for a
Python plugin used in the "example". 


## Prerequisites

To build the Rust code, you will need to install a recent version of `libzmq` (the ZeroMQ C++ library).
Version 4.1 (or newer) should suffice. 

On Debian/Ubuntu, you can get it by running this command:

```
$ apt-get install libzmq3-dev
```

## Running the example

The project currently includes an example with 4 plugins, 3 written in Rust and
compiled into the main engine, that use inproc sockets for communication, and one plugin written in Python
that uses TCP sockets. All plugins make use of Flatbuffers for message serialization.
To run the full example, you will need to build the Rust engine Docker image (with Rust plugins compiled in) as
well as the Python plugin Docker image. We include a Makefile with commands to make this easier:

```
# build the two Docker images
$ make build

# run the example
$ make up-engine
```


## Running the demo
There is also a more simple "demo" which includes three plugins, one in Rust and two in Python,
and that use simple strings for messages. We're not actively maintaining this "demo" since the 
"example" now contains plugins in multiple languages, but here is what *might* work for running
the demo:

Running the demo currently requires a small code
change to the `main.rs` file -- comment out the line that starts the example engine and recompile.

In order to run the demo, you also need to build the Python image in the `proto` directory. That
directory contains its own Makefile for building the Python image.

Once the Python image is build, use the Makefile in this directory to run the example:

```
$ make up-demo
```

## Development tasks
A collection of reminders for making code changes..

### Getting `flatc`

To make changes to the flatbuffers messages and regenerate the Rust code, you need the `flatc` binary. You
can build on your own machine. You can do that on Linux with the following:

1. clone the repo: 
  git clone https://github.com/google/flatbuffers.git

2. cd into coderepo and build with cmake:
  cmake -G "Unix Makefiles"
  make

3. Link binary and set permissions
  sudo ln -s /home/jstubbs/software/flatbuff/flatbuffers/flatc /usr/local/bin/flatc  
  chmod +x /home/jstubbs/software/flatbuff/flatbuffers/flatc



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


