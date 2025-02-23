// GET /
async function index(ctx, next) {
    ctx.body = 'Hello, koa server';
}

export default {
    'GET /': index
};
