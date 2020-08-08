import {lens} from "./optics.mjs";
import {createLogger} from "./logger.mjs";

const logger = createLogger("Router");

export const link = (page, menu) => {
  const effectiveMenu = menu ? menu : page.menus[0];
  return `#${page.key}/${effectiveMenu.key}`;
};

export const activeRoute$ = lens("activeRoute");

export class Router {
  constructor(store, pages) {
    this.store = store;
    this.pages = pages;
    // Build routes
    this.routes = pages.flatMap((page) =>
      page.menus.map((menu) => ({ page, menu }))
    );
    // Register events
    window.onhashchange = (event) => this.hashChanged(event);
  }

  hashChanged(event) {
    const { hash } = document.location;
    const fragments = hash.split("/");
    const currentPage = fragments.length > 0 ? fragments[0].substring(1) : "";
    const currentMenu = fragments[1] || null;
    const route = this.routes.find(
      (route) =>
        route.page.key === currentPage &&
        (currentMenu === null || route.menu.key === currentMenu)
    );
    logger.info("Change to route", { route });
    this.store.updateState(activeRoute$, route);
  }
}
