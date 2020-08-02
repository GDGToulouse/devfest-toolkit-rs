import { defaultRoute } from "./routes.mjs";

// Default State
export const defaultState = () => {
  const title = "DevFest Toolkit";
  const github = "https://github.com/GDGToulouse/devfest-toolkit-rs";
  const activeRoute = defaultRoute;

  return { title, activeRoute, github };
};
