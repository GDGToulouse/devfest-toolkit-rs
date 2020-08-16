import { Store } from "./fmwk/index.mjs";
import { effect } from "./reducer.mjs";
import { activeRoute$, defaultState } from "./state.mjs";

import "./routes/index.mjs";
import "./pages/index.mjs";
import "./components/index.mjs";

// Bootstrap application
const state = defaultState();
export const store = new Store(state);
store.connect(document, effect);

// Routing
const routesElt = document.querySelector("dftk-routes");
if (routesElt) {
  const setCurrentRoute = () => {
    const activeRoute = routesElt.findRoute(document.location);
    store.updateState(activeRoute$, activeRoute);
  };
  window.onhashchange = setCurrentRoute;
  setCurrentRoute();
}
