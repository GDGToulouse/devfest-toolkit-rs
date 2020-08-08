import {gqlQuery} from "./fmwk/graphql.mjs";
import {createLogger} from "./fmwk/logger.mjs";

const logger = createLogger("Effect");

export const effect = (store, action) => {
  logger.info(`handle`, { action });

  switch (action.key) {
    case "graphql-query":
      return queryGraphql(store, action);

    default:
      throw new Error(`Unknown action ~${action}`);
  }
};

const queryGraphql = async (store, { selector, query, projection }) => {
  const data = await gqlQuery(query);
  const value = projection ? data[projection] : data;
  store.updateState(selector, value);
};
