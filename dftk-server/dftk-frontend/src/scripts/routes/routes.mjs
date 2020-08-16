import { createLogger } from "../fmwk/logger.mjs";

export class RoutesElt extends HTMLElement {
  get routes() {
    return this._routes.reduce((acc, route) => {
      const key = route.label.toLowerCase();
      const menu = { key: route.menu.toLowerCase(), label: route.menu };
      const existingRoute = acc.find((r) => r.key === key);
      if (existingRoute) {
        existingRoute.menus.push(menu);
      } else {
        acc.push({ key, label: route.label, menus: [menu] });
      }
      return acc;
    }, []);
  }

  constructor() {
    super();
    this.logger = createLogger("RoutesElt");
  }

  // Lifecycle
  connectedCallback() {
    // this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });
    this._routes = [];
    this.addEventListener("route", (evt) => this.addRoute(evt.detail));
  }

  addRoute(newRoute) {
    if (newRoute && newRoute.label && newRoute.menu) {
      this._routes.push(newRoute);
      this.logger.debug("Routes", { routes: this.routes });
    }
  }

  findRoute(location) {
    const { hash } = document.location;
    const fragments = hash.split("/");
    const currentPage = fragments.length > 0 ? fragments[0].substring(1) : "";
    const currentMenu = fragments[1] || null;
    return this._routes.find(
      (route) =>
        currentPage === route.label.toLowerCase() &&
        currentMenu === route.menu.toLowerCase()
    );
  }
}
