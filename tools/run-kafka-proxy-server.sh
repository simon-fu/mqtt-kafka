#!/bin/bash

# refer https://github.com/grepplabs/kafka-proxy

CURR_DIR="$( cd "$( dirname $0)" && pwd )"
#echo CURR_DIR=$CURR_DIR
cd $CURR_DIR

# $CURR_DIR/kafka-proxy server \
#         --bootstrap-server-mapping "localhost:19092,0.0.0.0:30001" \
#         --bootstrap-server-mapping "localhost:29092,0.0.0.0:30002" \
#         --bootstrap-server-mapping "localhost:39092,0.0.0.0:30003" \
#         --dial-address-mapping "localhost:19092,127.0.0.1:9192" \
#         --dial-address-mapping "localhost:29092,127.0.0.1:9292" \
#         --dial-address-mapping "localhost:39092,127.0.0.1:9392" \
#         --dynamic-listeners-disable \
#         --debug-enable

$CURR_DIR/kafka-proxy server \
        --bootstrap-server-mapping "localhost:9192,0.0.0.0:30001" \
        --bootstrap-server-mapping "localhost:9292,0.0.0.0:30002" \
        --bootstrap-server-mapping "localhost:9392,0.0.0.0:30003" \
        --dial-address-mapping "ali-cn1-mqtt-kafka01:9092,127.0.0.1:9192" \
        --dial-address-mapping "ali-cn1-mqtt-kafka02:9092,127.0.0.1:9292" \
        --dial-address-mapping "ali-cn1-mqtt-kafka03:9092,127.0.0.1:9392" \
        --debug-enable
