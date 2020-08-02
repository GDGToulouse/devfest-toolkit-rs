import {createLogger} from "../src/scripts/fmwk/logger.mjs";

const logger = createLogger('assets');

export async function buildAssets(source, destination) {
    logger.warn(`TODO package assets '${source}' -> '${destination}'`);
}