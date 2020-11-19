#!/bin/bash

#This script assumes the crate has already been built successfully. 

cd ./target/avr-atmega32u4/debug/

avr-objcopy -S -j .text -j .data -O ihex ardurust.elf ardurust.hex

echo Press the reset button on the Arduboy now. 

sleep 1.5

avrdude -q -q -patmega32u4 -cavr109 -P/dev/ttyACM0 -b57600 -D -v -Uflash:w:ardurust.hex:i

cd ../../../../
