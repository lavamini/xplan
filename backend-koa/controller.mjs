import Router from '@koa/router';
import path from 'node:path';
import { readdirSync } from 'node:fs';
import { fileURLToPath } from 'url';

async function scan(router, controllerDir) {
    // Scan controller
    const dirname = path.dirname(fileURLToPath(import.meta.url));
    let files = readdirSync(path.join(dirname, controllerDir)).filter(f => f.endsWith('.mjs'));

    for (let file of files) {
        // Import controller
        let { default: mapping } = await import(`./${controllerDir}/${file}`);

        // Add handler to router
        for (let url in mapping) {
            if (url.startsWith('GET ')) {
                let p = url.substring(4);
                router.get(p, mapping[url]);
            } else if (url.startsWith('POST ')) {
                let p = url.substring(5);
                router.post(p, mapping[url]);
            } else if (url.startsWith('PUT ')) {
                let p = url.substring(4);
                router.put(p, mapping[url]);
            } else if (url.startsWith('DELETE ')) {
                let p = url.substring(7);
                router.del(p, mapping[url]);
            } else {
                console.warn(`invalid mapping: ${url}`);
            }
        }
    }
}

export default async function (controllerDir = 'controller') {
    const router = new Router();
    await scan(router, controllerDir);
    return router.routes();
}
