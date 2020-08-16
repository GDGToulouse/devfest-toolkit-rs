// Actions
export const createAction = (action) =>
  new CustomEvent("action", {
    detail: action,
    bubbles: true,
    composed: true,
  });


export const queryAction = (selector, query, projection) => ({
  key: "graphql-query",
  selector,
  query,
  projection,
});

export const generalAction = (id, action) => ({
  key: "general-action",
  id,
  action,
});

export const saveAction = (id, action) => ({
  key: "save-action",
  id,
  action,
});
