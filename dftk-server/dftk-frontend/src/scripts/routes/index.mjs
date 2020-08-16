import { CurrentRouteElt } from "./current-route.mjs";
import { RouteElt } from "./route.mjs";
import { RoutesElt } from "./routes.mjs";

customElements.define("dftk-routes", RoutesElt);
customElements.define("dftk-route", RouteElt);
customElements.define("dftk-current-route", CurrentRouteElt);
