const Koa = require('koa');
const bodyParser = require('koa-bodyparser');
const config = require('config')
const mysql = require('mysql2');
const { program } = require('commander');

const registerRouter = require('./router');

program
    .option('-p, --port <port number>')
    .parse();

const options = program.opts();
const port = options.port ? parseInt(options.port) : 3000;

const app = new Koa();
app.use(bodyParser());
app.use(registerRouter('router'))

const db_config = config.get('db');
app.context.db_pool  = mysql.createPool({
    host: db_config['host'] || 'localhost',
    port: db_config['port'] || 3306,
    user: db_config['user'] || 'root',
    password: db_config['password'],
    database: db_config['database'],
    maxIdle: db_config['min_connections'] || 10,
    connectionLimit: db_config['max_connections'] || 10,
});

console.log('⇨ koa server listening on \x1b[32m' + port + '\x1b[0m');
app.listen(port);
