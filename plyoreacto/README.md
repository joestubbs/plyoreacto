# Rust Engine
This directory contains the code for the Rust engine.

## Prerequisites

To build, need to install the dependencies:

On Ubuntu, 
```
$ apt-get install libzmq3-dev
```

## Running the example
In order to run the example, you also need to build the Python image in the `proto` directory. That
directory contains its own Makefile for building the Python image.

Once the Python image is build, use the Makefile in this directory to run the example:

```
$ make all
```



