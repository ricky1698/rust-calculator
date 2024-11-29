#!/bin/bash

sudo apt-get update && sudo apt-get install -y \
    build-essential \
    cmake \
    gdb \
    lldb \
    clang \
    libclang-dev \
    pkg-config \
    lcov

brew install just