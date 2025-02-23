// POST /signin
async function signin(ctx, next) {
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
}

export default {
    'POST /api/signin': signin
};
