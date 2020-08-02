import {createLogger} from "../src/scripts/fmwk/logger.mjs";

const logger = createLogger('scripts');

export async function buildScripts(source, destination) {
    logger.warn(`TODO package JavaScript from path  '${source}' -> '${destination}'`);
}