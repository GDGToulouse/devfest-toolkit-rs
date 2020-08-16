import { lens } from "./fmwk/index.mjs";

export const activeRoute$ = lens("activeRoute");

// Default State
export const defaultState = () => {
  const activeRoute = { key: "general", menu: "information" };

  return { activeRoute };
};
