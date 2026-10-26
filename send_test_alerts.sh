#!/usr/bin/env bash

curl 127.0.0.1:3000 -H "Content-Type: application/json" --json @fixtures/AwsBuilderTerminationFailed.json
curl 127.0.0.1:3000 -H "Content-Type: application/json" --json @fixtures/ServerCPU.json
curl 127.0.0.1:3000 -H "Content-Type: application/json" --json @fixtures/ServerDisk.json
curl 127.0.0.1:3000 -H "Content-Type: application/json" --json @fixtures/ServerMem.json
curl 127.0.0.1:3000 -H "Content-Type: application/json" --json @fixtures/StackAutoUpdated.json
curl 127.0.0.1:3000 -H "Content-Type: application/json" --json @fixtures/StackImageUpdateAvailable.json
curl 127.0.0.1:3000 -H "Content-Type: application/json" --json @fixtures/Test.json