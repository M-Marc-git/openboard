#!/bin/env bash

mkdir -p db
touch db/project.db

cargo build --release
