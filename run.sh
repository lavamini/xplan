if [ $# -eq 0 ]; then
    printf "\x1b[31mUsage:\x1b[0m ./run.sh <axum | fiber | koa>\n"
else
    if [ "$1" == "axum" ]; then
        cd backend-axum
        cargo build --release
        cp ./target/release/backend-axum ./main
        RUST_LOG=info ./main
        cd ..
    elif [ "$1" == "fiber" ]; then
        cd backend-fiber
        go build -ldflags '-w -s' main.go router.go
        ./main
        cd ..
    elif [ "$1" == "koa" ]; then
        cd backend-koa
        node app.js
        cd ..
    else
        printf "\x1b[31mUsage:\x1b[0m ./run.sh <axum | fiber | koa>\n"
    fi
fi
