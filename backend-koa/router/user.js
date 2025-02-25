const Router = require('koa-router');
const bcrypt = require('bcryptjs');
const moment = require('moment');

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
                    if (error) {
                        console.warn(error);
                        resolve(undefined);
                    }
                    resolve(results);
                }
            );
        });

        if (results == undefined) {
            ctx.body = { code: 1, msg: 'signin failed' };
        } else if (results.length == 0) {
            ctx.body = { code: 1, msg: 'name or password not correct' };
        } else {
            const password_hash = results[0].password_hash.toString();
            // async password compare
            const b_match = await new Promise(resolve => {
                bcrypt.compare(password, password_hash, (err, b_match) => {
                    if (err) {
                        console.warn(err);
                        resolve(undefined);
                    }
                    resolve(b_match);
                });
            });

            if (b_match == undefined) {
                ctx.body = { code: 1, msg: 'signin failed' };
            } else if (b_match) {
                ctx.body = { code: 0, msg: 'signin success' };
            } else {
                ctx.body = { code: 1, msg: 'name or password not correct' };
            }
        }
    }
});

// signup
router.post('/signup', async (ctx, next) => {
    let name = ctx.request.body.name || '';
    let password = ctx.request.body.password || '';

    name = name.trim();
    password = password.trim();

    if (name == '' || password == '') {
        ctx.body = { code: 1, msg: 'Parameters missing' };
    } else {
        // async db query
        const match_rows = await new Promise(resolve => {
            ctx.db_pool.query(
                "SELECT id FROM user WHERE name='" + name + "'",
                (error, results, fields) => {
                    if (error) {
                        console.warn(error);
                        resolve(-1);
                    }
                    resolve(results.length);
                }
            );
        });

        if (match_rows == -1) {
            ctx.body = { code: 1, msg: 'signup failed' };
        } else if (match_rows > 0) {
            ctx.body = { code: 1, msg: 'name already exist' };
        } else {
            // async password hash
            const password_hash = await new Promise(resolve => {
                bcrypt.hash(password, 10, function(err, hash) {
                    if (err) {
                        console.warn(err);
                        resolve("");
                    }
                    resolve(hash);
                });
            });

            if (password_hash == "") {
                ctx.body = { code: 1, msg: 'signup failed' };
            } else {
                const created_at = moment().format('YYYY-MM-DD HH:mm:ss');
                const user = {
                    name,
                    password_hash,
                    created_at,
                    updated_at: created_at
                };

                // async db insert
                const insertId = await new Promise(resolve => {
                    ctx.db_pool.query(
                        'INSERT INTO user SET ?', user,
                        (error, results, fields) => {
                            if (error) {
                                console.warn(error);
                                resolve(0);
                            }
                            resolve(results.insertId);
                        }
                    );
                });

                if (insertId > 0) {
                    ctx.body = { code: 0, msg: 'signup success' };
                } else {
                    ctx.body = { code: 1, msg: 'signup failed' };
                }
            }
        }
    }
});

// users
router.get('/users', async (ctx, next) => {
    // async db query
    const results = await new Promise(resolve => {
        ctx.db_pool.query(
            'SELECT id, name, created_at, updated_at FROM user',
            (error, results, fields) => {
                if (error) {
                    console.warn(error);
                    resolve(undefined);
                }
                resolve(results);
            }
        );
    });

    if (results == undefined) {
        ctx.body = { code: 1, msg: 'get users failed' };
    } else {
        for (let i in results) {
            results[i].name = results[i].name.toString();
            results[i].created_at = moment(results[i].created_at).format('YYYY-MM-DD HH:mm:ss');
            results[i].updated_at = moment(results[i].updated_at).format('YYYY-MM-DD HH:mm:ss');
        }

        ctx.body = {
            code: 0,
            data: results,
            msg: 'success'
        }
    }
});

module.exports = router;
