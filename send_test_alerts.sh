#!/usr/bin/env bash

curl localhost:3000 -H "Content-Type: application/json" --json @fixtures/AwsBuilderTerminationFailed.json
curl localhost:3000 -H "Content-Type: application/json" --json @fixtures/ServerCPU.json
curl localhost:3000 -H "Content-Type: application/json" --json @fixtures/ServerDisk.json
curl localhost:3000 -H "Content-Type: application/json" --json @fixtures/ServerMem.json
curl localhost:3000 -H "Content-Type: application/json" --json @fixtures/StackAutoUpdated.json
curl localhost:3000 -H "Content-Type: application/json" --json @fixtures/StackImageUpdateAvailable.json
curl localhost:3000 -H "Content-Type: application/json" --json @fixtures/Test.json