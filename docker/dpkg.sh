#!/bin/sh
error_exit() {
    echo "cd failed"
    exit 1
}

cd "/glimpse" || error_exit

echo "files"

ls -la

cargo install cargo-zigbuild

cd "scripts" || error_exit

ls -la

echo "start compile"

sh ./compile-all.sh --linux-dpkg

result=$?

if [ "$result" -ne 0 ]
then
    echo "${RED} compile failed"
    exit 1
fi

echo "start actual dpkg build"
ls "/glimpse"
ls "/glimpse/dist"

dpkg --build /glimpse/dist/glimpse-1.0.0-linux-x64

result=$?

if [ "$result" -ne 0 ]
then
    echo "${RED} dpkg failed"
    exit 1
fi

# shellcheck disable=SC2067
find /glimpse -name "*.deb" -exec cp {} /glimpse/host

#mkdir -p /output && find /glimpse -name "*.deb" -exec cp {} /output/ \;