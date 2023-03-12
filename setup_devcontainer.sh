#!/bin/bash

# Install LLVM v15.x.y
apt-get update
apt-get install -y lsb-release wget software-properties-common gnupg

wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
./llvm.sh 15
rm -rf ./llvm.sh

apt-get install -y libpolly-15-dev