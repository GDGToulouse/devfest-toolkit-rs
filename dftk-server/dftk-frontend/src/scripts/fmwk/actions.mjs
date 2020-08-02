// Actions
export const createAction = (action) =>
  new CustomEvent("action", {
    detail: action,
    bubbles: true,
  });

export const queryAction = (selector, query, projection) => ({
  key: "graphql-query",
  selector,
  query,
  projection,
});
