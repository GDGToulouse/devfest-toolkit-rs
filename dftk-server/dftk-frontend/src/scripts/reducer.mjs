import { gqlQuery, createLogger } from "./fmwk/index.mjs";

const logger = createLogger("Effect");

export const effect = (store, action) => {
  logger.info(`handle`, { action });

  switch (action.key) {
    case "graphql-query":
      return queryGraphql(store, action);

    case "general-action":
      return queryGraphql(store, { query: `mutation { ${action.action} {} }` });

    case "save-action":
      return queryGraphql(store, { query: `mutation { ${action.action} {} }` });

    default:
      throw new Error(`Unknown action ~${action}`);
  }
};

const queryGraphql = async (store, { selector, query, projection }) => {
  const data = await gqlQuery(query);
  if (selector) {
    const value = projection ? data[projection] : data;
    store.updateState(selector, value);
  }
};
