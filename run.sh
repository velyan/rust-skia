#!/usr/bin/env bash

set -eu

./perceptualdiff $1 $2 -output $3
#-threshold 500
