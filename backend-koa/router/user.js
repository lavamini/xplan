const Router = require('koa-router');
const bcrypt = require('bcryptjs');

const router = new Router();
router.prefix('/api');

// signin
router.post('/signin', async (ctx, next) => {
    let name = ctx.request.body.name || '';
    let password = ctx.request.body.password || '';

    name = name.trim();
    password = password.trim();

    if (name == '' || password == '') {
        ctx.body = { code: 1, msg: 'Parameters missing' };
    } else {
        // async db query
        const results = await new Promise(resolve => {
            ctx.db_pool.query(
                "SELECT password_hash FROM user WHERE name='" + name + "'",
                (error, results, fields) => {
                    if (error) throw error;
                    resolve(results);
                }
            );
        });

        if (results.length == 0) {
            ctx.body = { code: 1, msg: 'name or password not correct' };
        } else {
            const password_hash = results[0].password_hash.toString();
            // async password compare
            const b_match = await new Promise(resolve => {
                bcrypt.compare(password, password_hash, (err, b_match) => {
                    if (err) throw err;
                    resolve(b_match);
                });
            });

            if (b_match) {
                ctx.body = { code: 0, msg: 'signin success' };
            } else {
                ctx.body = { code: 1, msg: 'name or password not correct' };
            }
        }
    }
});

module.exports = router;
