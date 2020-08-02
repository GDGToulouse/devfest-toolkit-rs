import {createLogger} from "../src/scripts/fmwk/logger.mjs";

const logger = createLogger('styles');

export async function buildStyles(source, destination) {
    logger.warn(`TODO package Styles from path '${source}' -> '${destination}'`);
}