#!/bin/bash

# Install LLVM v15.x.y
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
./llvm.sh 15
apt-get install libpolly-15-dev