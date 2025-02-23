const Router = require('koa-router');

const router = new Router();
router.prefix('/api');

// signin
router.post('/signin', (ctx, next) => {
    let name = ctx.request.body.name || '';
    let password = ctx.request.body.password || '';

    if (name === '' || password === '') {
        ctx.body = {
            code: 1,
            msg: 'Parameters missing'
        };
    } else {
        ctx.body = {
            code: 0,
            msg: 'Success'
        };
    }
});

module.exports = router;
