// GET /
async function index(ctx, next) {
    ctx.response.type = 'text/plain';
    ctx.response.body = 'Hello, koa server';
}

// POST /login
async function login(ctx, next) {
    ctx.response.type = 'application/json;charset=utf-8';

    let name = ctx.request.body.name || '';
    let password = ctx.request.body.password || '';

    if (name === '' || password === '') {
        ctx.response.body = {
            code: 1,
            msg: 'Parameters missing'
        };
    } else {
        ctx.response.body = {
            code: 0,
            msg: 'Success'
        };
    }
}

export default {
    'GET /': index,
    'POST /login': login
};