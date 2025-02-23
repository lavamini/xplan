import Koa from 'koa';
import Router from '@koa/router';
import { bodyParser } from '@koa/bodyparser';
import { program } from 'commander';

program
    .option('-p, --port <port number>')
    .parse();

const options = program.opts();
const port = options.port ? parseInt(options.port) : 3000;

const app = new Koa();
const router = new Router();

// Parse request.body
app.use(bodyParser());

// Set handle /
router.get('/', async (ctx, next) => {
    ctx.response.type = 'text/plain';
    ctx.response.body = 'Hello, koa server';
});

// Use router
app.use(router.routes());

console.log('⇨ koa server listening on \x1b[32m' + port + '\x1b[0m');
app.listen(port);
