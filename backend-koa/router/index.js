const Router = require('koa-router');

const router = new Router();

// index
router.get('/', (ctx, next) => {
    ctx.body = 'Hello, koa server';
});

module.exports = router;
