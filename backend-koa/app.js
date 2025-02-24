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
const db_pool = pool  = mysql.createPool(db_config);
app.context.db_pool = db_pool;

console.log('⇨ koa server listening on \x1b[32m' + port + '\x1b[0m');
app.listen(port);
