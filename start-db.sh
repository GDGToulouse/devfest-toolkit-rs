#!/usr/bin/env bash

echo "🐳 Start MongoDB 🍃"
docker run -p 27017:27017 -v /tmp/data:/data/db docker.io/mongo:3.6.14