#!/bin/bash

if [ $# -eq 0 ]; then
    printf "Usage: ./run.sh <axum | fiber | koa>\n"
else
    target=$1

    if [ $target == "axum" ]; then
        cd backend-axum
        cargo build --release
        cp ./target/release/backend-axum ./main
        RUST_LOG=info ./main
        cd ..
    elif [ $target == "fiber" ]; then
        cd backend-fiber
        go build -ldflags '-w -s' main.go router.go
        ./main
        cd ..
    elif [ $target == "koa" ]; then
        cd backend-koa
        node app.js
        cd ..
    else
        printf "Usage: ./run.sh <axum | fiber | koa>\n"
    fi
fi
