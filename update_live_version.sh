#!/bin/bash

cargo +nightly web deploy

if [ $? -eq 0 ]
then
  cp target/deploy/* live/
  echo [!] Updated live version
fi
