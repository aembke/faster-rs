#!/bin/bash

#docker-compose -f tests/docker/debian.yml run -u $(id -u ${USER}):$(id -g ${USER}) --rm faster_dev
docker-compose -f tests/docker/debian.yml run --rm faster_dev