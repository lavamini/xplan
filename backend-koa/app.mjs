import Koa from 'koa';
import { program } from 'commander';

program
    .option('-p, --port <port number>')
    .parse();

const options = program.opts();
const port = options.port ? parseInt(options.port) : 3000;

const app = new Koa();

app.use(async ctx => {
    ctx.body = 'Hello, koa server';
});

console.log('⇨ koa server listening on \x1b[32m' + port + '\x1b[0m');
app.listen(port);
