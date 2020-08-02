import {createLogger} from "../src/scripts/fmwk/logger.mjs";

const logger = createLogger('clean');

export async function clean(path) {
    logger.warn(`TODO cleanup path '${path}'`);
}