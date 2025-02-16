#!/bin/zsh

# Docker Build Package
docker build ./ --file ./hyaena-technologies.Dockerfile --tag amd64/hyaena-technologies-server:latest
