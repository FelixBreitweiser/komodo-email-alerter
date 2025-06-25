#!/usr/bin/env bash

curl localhost:3000 -H "Content-Type: application/json" --json @res/AwsBuilderTerminationFailed.json
curl localhost:3000 -H "Content-Type: application/json" --json @res/ServerCPU.json
curl localhost:3000 -H "Content-Type: application/json" --json @res/ServerDisk.json
curl localhost:3000 -H "Content-Type: application/json" --json @res/ServerMem.json
curl localhost:3000 -H "Content-Type: application/json" --json @res/StackAutoUpdated.json
curl localhost:3000 -H "Content-Type: application/json" --json @res/StackImageUpdateAvailable.json
curl localhost:3000 -H "Content-Type: application/json" --json @res/Test.json