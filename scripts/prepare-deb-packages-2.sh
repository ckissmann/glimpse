#!/bin/sh
remove_codebase() {
    echo "remove codebase"
    rm -rf ./docker/codebase
}

error_exit() {
    echo "cd failed"
    exit 1
}

cd "$(dirname "$0")" || error_exit
cd ".."

# copy all to /docker/codebase, because i cant select the parent folder in Dockerfile !?
echo "Copy codebase"
remove_codebase
mkdir ./docker/codebase
cp -R ./* ./docker/codebase/

result=$?

if [ "$result" -ne 0 ]
then
    echo "${RED} cp failed"
    remove_codebase
    exit 1
fi

cd "docker" || error_exit

docker compose -f "docker-compose-build-dpkg.yaml" up --build

result=$?

if [ "$result" -ne 0 ]
then
    echo "${RED} build failed"
    remove_codebase
    exit 1
fi

remove_codebase