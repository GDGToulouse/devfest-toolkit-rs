import { activateLink } from "../../dom-helpers.mjs";
import { createLogger } from "../../fmwk/logger.mjs";
import { activeRoute$ } from "../../state.mjs";

const CSS = `
:host {
    padding-left: .25rem;
    background-color: var(--app-selected-bg);
    color: var(--app-selected-fg);
    flex: 0 0 auto;
    display: flex;
    flex-direction: column;
}

ul {
    margin: 0;
    padding: 1em 0 0;
}
ul:empty {
    display: none;
}

li {
    list-style: none;
    padding: .5em 1em;
    border-bottom-left-radius: .5em;
    border-top-left-radius: .5em;
}

li a {
    color: inherit;
    text-decoration: none;
}

li a:hover, li a:active {
    text-decoration: underline;
}

li.active {
    background-color: var(--app-content-bg);
}
`;

export class SubmenuElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("SubmenuElt");
    // Elements
    this.menuElt = document.createElement("ul");
    this.menuElt.setAttribute("role", "");
    this.menuElt.setAttribute("aria-label", "Sub menu");
  }

  // Lifecycle
  connectedCallback() {
    // this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });
    this._currentRoute = null;
    this.render();

    // Register events
    document.addEventListener("store", (evt) =>
      evt.detail.register(activeRoute$, (route) => this.updateRoute(route))
    );
  }

  // Render
  render() {
    // this.logger.debug(`render`);

    const styleElt = document.createElement("style");
    styleElt.innerHTML = CSS;

    this.shadowRoot.append(styleElt, this.menuElt);
  }

  updateRoute(route) {
    if (route) {
      if (this._currentRoute !== route) {
        const routesElt = document.querySelector("dftk-routes");
        const activeRoute = (routesElt ? routesElt.routes : []).find(
          (it) => it.label === route.label
        );
        const menus = activeRoute ? activeRoute.menus : [];

        this.menuElt.innerHTML = menus
          .map((menu) => {
            const { key, label } = menu;
            const target = `#${route.label.toLowerCase()}/${key}`;
            return `<li role="none" class="key-${key}"><a href="${target}" role="menuitem">${label}</a></li>`;
          })
          .join("");
      }

      activateLink(this.shadowRoot, route.menu.toLowerCase());
    }
  }
}
