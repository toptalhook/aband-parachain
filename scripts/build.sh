#!/usr/bin/env bash
set -e

pushd .

# The following line ensure we run from the project root
#
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT && cd ..

# Find the current version from Cargo.toml
VERSION=`grep "^version" ./substrate-parachain-PoS-template/runtime/Cargo.toml | egrep -o "([0-9\.]+-?[0-9]+)"`
GITUSER=aband
GITREPO=mainnet-node

# Build the image
echo "Building ${GITUSER}/${GITREPO}:latest docker image, hang on!"
time docker build \
    -f ./substrate-parachain-PoS-template/scripts/node_builder.Dockerfile \
    -t ${GITUSER}/${GITREPO}:latest \
    -t ${GITUSER}/${GITREPO}:v${VERSION} \
    .

# Show the list of available images for this repo
echo "Your Docker image for $GITUSER/$GITREPO is ready"
docker images | grep ${GITREPO}

popd
