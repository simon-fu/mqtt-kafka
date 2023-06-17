#!/bin/bash

CURR_DIR="$( cd "$( dirname $0)" && pwd )"
#echo CURR_DIR=$CURR_DIR
cd $CURR_DIR

ssh \
  -L 127.0.0.1:9192:ali-cn1-mqtt-kafka01:9092 \
  -L 127.0.0.1:9292:ali-cn1-mqtt-kafka02:9092 \
  -L 127.0.0.1:9392:ali-cn1-mqtt-kafka03:9092 \
  mqtt-console

