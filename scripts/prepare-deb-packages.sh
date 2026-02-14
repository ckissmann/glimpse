#!/bin/sh

cd "$(dirname "$0")" || exit
cd ".."

# copy all to /docker/codebase, because i cant select the parent folder in Dockerfile !?
rm -rf ./docker/codebase
mkdir ./docker/codebase
cp -R ./* ./docker/codebase/

docker build -t glimpse-builder ./docker

result=$?

if [ "$result" -ne 0 ]
then
    echo "${RED} build failed"
    rm -rf ./docker/codebase
    exit 1
fi

docker run -v "$(pwd)/output:/output" glimpse-builder

rm -rf ./docker/codebase