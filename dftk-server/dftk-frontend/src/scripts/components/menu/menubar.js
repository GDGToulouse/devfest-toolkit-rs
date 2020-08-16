import { activateLink } from "../../dom-helpers.mjs";
import { createLogger } from "../../fmwk/logger.mjs";
import { activeRoute$ } from "../../state.mjs";

const CSS = `
:host {
  flex: 1 0 auto;
}

ul {
    margin: 0 1em;
    display: flex;
    flex-direction: row;
    justify-content: space-evenly;
    flex: 1 0 auto;
    --angle: 10deg;
}

li {
    padding: .5em .25em;
    list-style: none;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--darken);
    flex: 1 1 auto; 
    transform: skewX(calc(-1 * var(--angle)));
}

li a {
    padding: .5em .25em;
    color: inherit;
    text-decoration: none;
    transform: skewX(var(--angle));
}

li a:visited {
    color: inherit;
}

li a:hover, li a:active {
    text-decoration: underline;
}

li.active {
    background-color: var(--app-selected-bg);
    color: var(--app-selected-fg);
    border: thin solid var(--app-selected-bg);
}
`;

export class MenubarElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("MenubarElt");
  }

  // Lifecycle
  connectedCallback() {
    // this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });
    this.render();

    // Register events
    document.addEventListener("store", (evt) =>
      evt.detail.register(activeRoute$, (route) => this.updateRoute(route))
    );
  }

  // Render
  render() {
    // this.logger.debug(`render`);

    // Menu
    const routesElt = document.querySelector("dftk-routes");
    const routes = routesElt ? routesElt.routes : [];
    const menus = routes
      .map((route) => {
        const { key, label, menus } = route;
        const target = menus.length ? `#${key}/${menus[0].key}` : `#${key}`;
        return `<li role="none" class="key-${key}"><a href="${target}" role="menuitem">${label}</a></li>`;
      })
      .join("");

    // Construction
    this.shadowRoot.innerHTML = `
<style>${CSS}</style>
<ul role="menubar" aria-label="Main Menubar">${menus}</ul>
`;
  }

  updateRoute(route) {
    if (route) {
      activateLink(this.shadowRoot, route.label.toLowerCase());
    }
  }
}
