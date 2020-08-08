import {createLogger} from "./logger.mjs";

const logger = createLogger("GraphQL");

const url = "http://localhost:8080/graphql";

export const gqlQuery = async (query, variables) => {
  const req = {
    method: "POST",
    mode: "cors",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ query, variables }),
  };
  logger.info(`query to ${url}`, req);
  const response = await fetch(url, req);
  const { data, errors } = await response.json();
  logger.debug(`response`, { data, errors });
  (errors || []).forEach((err) => logger.error("receive error", err));

  return data;
};
