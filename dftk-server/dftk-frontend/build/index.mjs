import {clean} from "./clean.mjs";
import {buildAssets} from "./assets.mjs";
import {buildStyles} from "./styles.mjs";
import {buildScripts} from "./scripts.mjs";
import {createLogger} from "../src/scripts/fmwk/logger.mjs";

const logger = createLogger('build');

const config = {
    output: './dist',
    assets: './src/assets',
    scripts: './src/scripts',
    styles: './src/styles'
}

export async function build(config) {
    const {output, assets, scripts, styles} = config;

    await clean(output);
    await buildAssets(output);
    await buildStyles(output);
    await buildScripts(output);
}

logger.info("building...")
build(config)
    .then(() => logger.info("✅ built."));
