## Integrate multiple backend APIs with Go, Node.js and Rust

### database
```sh
docker compose up -d
```

### run
```sh
./run.sh axum
--- or ---
./run.sh fiber
--- or ---
./run.sh koa
```

### api
```sh
GET http://localhost:3000/
```
```sh
POST http://localhost:3000/api/signup
{
    "name": "Jay",
    "password": "testpass"
}
```
```sh
POST http://localhost:3000/api/signin
{
    "name": "Jay",
    "password": "testpass"
}
```
```sh
GET http://localhost:3000/api/users?page=1&page_size=20
```
```sh
GET http://localhost:3000/api/employees?page=1&page_size=20
```
