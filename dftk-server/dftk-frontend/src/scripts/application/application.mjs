import {
  activateLink,
  createMenubar,
  createMenuItem,
} from "../dom-helpers.mjs";
import { activeRoute$, link } from "../fmwk/routes.mjs";
import { createLogger } from "../fmwk/logger.mjs";

const CSS = `:host {
    display: grid;
    grid-template-rows: auto 1fr auto;
    width: 100%;
    background-color: var(--app-selected-bg);
    box-sizing: border-box;
}

/* Navbar */
:host > nav {
    background-color: var(--app-navbar-bg);
    color: var(--app-navbar-fg, #ddd);
    display: flex;
}

:host > nav > h1 {
    margin: .5em 1em;
    flex: 0 1 auto;
    display: flex;
    align-items: center;
}

/* Menubar */
.menubar {
    margin: 0 1em;
    display: flex;
    justify-content: space-evenly;
    flex: 1 0 auto;
    --angle: 10deg;
}

.menubar > li {
    padding: .5em 1em;
    list-style: none;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--darken);
    flex: 1 1 auto;
    transform: skewX(calc(-1 * var(--angle)));
}

.menubar > li > a {
    padding: .5em 1em;
    color: inherit;
    text-decoration: none;
    transform: skewX(var(--angle));
}

.menubar > li > a:visited {
    color: inherit;
}

.menubar > li > a:hover, .menubar > li > a:active {
    text-decoration: underline;
}

.menubar > li.active {
    background-color: var(--app-selected-bg);
    color: var(--app-selected-fg);
    border: thin solid var(--app-selected-bg);
}

/* Content */
:host > div {
    margin: .5em .25em .25em;
    background-color: var(--app-selected-bg);
    color: var(--app-content-fg);
    border-radius: .25em;
    display: flex;
}

/* Submenu */
.submenu {
    margin: 0;
    padding: 1em 0 0;
    background-color: var(--app-selected-bg);
    color: var(--app-selected-fg);
    flex: 0 0 auto;
    display: flex;
    flex-direction: column;
    min-width: 16ch;
}

.submenu:empty {
    display: none;
}

.submenu li {
    list-style: none;
    padding: .5em 1em;
    border-bottom-left-radius: .5em;
    border-top-left-radius: .5em;
}

.submenu li a {
    color: inherit;
    text-decoration: none;
}

.submenu li a:hover, .submenu li a:active {
    text-decoration: underline;
}

.submenu li.active {
    background-color: var(--app-content-bg);
}

/* Page */
.page {
    border-radius: .25em;
    background-color: var(--app-content-bg);
    overflow: auto;
    flex: 1 1 auto;
}

/* Footer */
:host > footer {
    background-color: var(--app-footer-bg);
    color: var(--app-footer-fg);
    min-height: 1.2em;
    padding: .25em 1em;
    text-align: left;
    display: flex;
    justify-content: space-between;
}

:host > footer::after {
    content: 'We ❤️️ chocolatines';
}


/* Github Link */
.github {
    color: var(--action-text);
    display: flex;
    text-decoration: none;
}
.github:visited {
    color: inherit;
}
.github:hover, .github:active {
    text-decoration: underline;
}

.github::before {
    content: ' ';
    width: 1.25em;
    height: 1.25em;
    margin-right: .5em;
    background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAyRpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDUuNS1jMDIxIDc5LjE1NDkxMSwgMjAxMy8xMC8yOS0xMTo0NzoxNiAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6RERCMUIwOUY4NkNFMTFFM0FBNTJFRTMzNTJEMUJDNDYiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6RERCMUIwOUU4NkNFMTFFM0FBNTJFRTMzNTJEMUJDNDYiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENTNiAoTWFjaW50b3NoKSI+IDx4bXBNTTpEZXJpdmVkRnJvbSBzdFJlZjppbnN0YW5jZUlEPSJ4bXAuaWlkOkU1MTc4QTJBOTlBMDExRTI5QTE1QkMxMDQ2QTg5MDREIiBzdFJlZjpkb2N1bWVudElEPSJ4bXAuZGlkOkU1MTc4QTJCOTlBMDExRTI5QTE1QkMxMDQ2QTg5MDREIi8+IDwvcmRmOkRlc2NyaXB0aW9uPiA8L3JkZjpSREY+IDwveDp4bXBtZXRhPiA8P3hwYWNrZXQgZW5kPSJyIj8+jUqS1wAAApVJREFUeNq0l89rE1EQx3e3gVJoSPzZeNEWPKgHoa0HBak0iHiy/4C3WvDmoZ56qJ7txVsPQu8qlqqHIhRKJZceesmhioQEfxTEtsoSpdJg1u/ABJ7Pmc1m8zLwgWTmzcw3L+/te+tHUeQltONgCkyCi2AEDHLsJ6iBMlgHL8FeoqokoA2j4CloRMmtwTmj7erHBXPgCWhG6a3JNXKdCiDl1cidVbXZkJoXQRi5t5BrxwoY71FzU8S4JuAIqFkJ2+BFSlEh525b/hr3+k/AklDkNsf6wTT4yv46KIMNpsy+iMdMc47HNWxbsgVcUn7FmLAzzoFAWDsBx+wVP6bUpp5ewI+DOeUx0Wd9D8F70BTGNjkWtqnhmT1JQAHcUgZd8Lo3rQb1LAT8eJVUfgGvHQigGp+V2Z0iAUUl8QH47kAA1XioxIo+bRN8OG8F/oBjwv+Z1nJgX5jpdzQDw0LCjsPmrcW7I/iHScCAEDj03FtD8A0EyuChHgg4KTlJQF3wZ7WELppnBX+dBFSVpJsOBWi1qiRgSwnOgoyD5hmuJdkWCVhTgnTvW3AgYIFrSbZGh0UW/Io5Vp+DQoK7o80pztWMemZbgxeNwCNwDbw1fIfgGZjhU6xPaJgBV8BdsMw5cbZoHsenwYFxkZzl83xTSKTiviCAfCsJLysH3POfC8m8NegyGAGfLP/VmGmfSChgXroR0RSWjEFv2J/nG84cuKFMf4sTCZqXuJd4KaXFVjEG3+tw4eXbNK/YC9oXXs3O8NY8y99L4BXY5cvLY/Bb2VZ58EOJVcB18DHJq9lRsKr8inyKGVjlmh29mtHs3AHfuhCwy1vXT/Nu2GKQt+UHsGdctyX6eQyNvc+5sfX9Dl7Pe2J/BRgAl2CpwmrsHR0AAAAASUVORK5CYII=);
    background-size: contain;
}`;

export class ApplicationElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("ApplicationElt");

    // Elements
    this.titleElt = document.createElement("h1");

    this.pageListElt = createMenubar("main menu bar");
    this.pageListElt.classList.add("menubar");

    this.menuListElt = createMenubar("sub menu bar");
    this.menuListElt.classList.add("submenu");

    this.pageContainerElt = document.createElement("div");
    this.pageContainerElt.classList.add("page");

    this.githubElt = document.createElement("a");
    this.githubElt.classList.add("github");
  }

  connect(router) {
    if (this._connected) {
      this.logger.error(`Already connected!`);
    }
    this.logger.info(`connect to`, { router });

    this._connected = true;
    this.store = router.store;
    this.router = router;

    if (this._init) {
      this.firstRendering();
    }
  }

  // Lifecycle
  connectedCallback() {
    this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });
    this.init();
  }

  adoptedCallback() {
    this.logger.debug(`adopted`);
  }

  disconnectedCallback() {
    this.logger.debug(`disconnected`);
  }

  // Attributes
  static get observedAttributes() {
    return [];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    this.logger.debug(`attribute '${name} changed`, { oldValue, newValue });
  }

  // Listener

  // Render
  init() {
    if (this._init) {
      this.logger.error(`🚨 [${this.name}] Already initialized!`);
    }
    this.logger.info(`initialize`);

    // Style
    const styleElt = document.createElement("style");
    styleElt.innerHTML = CSS;

    // Navbar
    const navbarElt = document.createElement("nav");
    navbarElt.appendChild(this.titleElt);
    navbarElt.appendChild(this.pageListElt);

    // Content
    const contentElt = document.createElement("div");
    contentElt.appendChild(this.menuListElt);
    contentElt.appendChild(this.pageContainerElt);

    // Footer
    const footerElt = document.createElement("footer");
    footerElt.appendChild(this.githubElt);

    // Construction
    this.shadowRoot.appendChild(styleElt);
    this.shadowRoot.appendChild(navbarElt);
    this.shadowRoot.appendChild(contentElt);
    this.shadowRoot.appendChild(footerElt);

    // Register internal events

    // First Render
    this._init = true;
    if (this._connected) {
      this.firstRendering();
    }
  }

  firstRendering() {
    // Render
    this.store.state.then((state) =>
      this.renderState(state, this.router.pages)
    );

    // Register events
    this.store.register(activeRoute$, (route) => this.updateRoute(route));
  }

  renderState(state, pages) {
    const { title, github, activeRoute } = state;

    this.titleElt.innerText = title;
    this.renderPages(pages, activeRoute);
    this.updateRoute(activeRoute);

    // Footer
    this.githubElt.href = github;
    const index = github.lastIndexOf("/");
    this.githubElt.textContent = github.substring(index + 1);
  }

  renderPages(pages, activeRoute) {
    this.pageListElt.innerHTML = "";
    pages.forEach((page) => {
      const { key, label } = page;
      const href = link(page);
      const elt = createMenuItem(key, label, href);
      this.pageListElt.appendChild(elt);
    });
  }

  renderPageMenu(activeRoute) {
    this.menuListElt.innerHTML = "";
    const menus = activeRoute ? activeRoute.page.menus : [];
    menus.forEach((menu) => {
      const { key, label } = menu;
      const href = link(activeRoute?.page, menu);
      const elt = createMenuItem(key, label, href);
      this.menuListElt.appendChild(elt);
    });
  }

  updateRoute(route) {
    this.logger.info(`updateRoute`, { route });
    // Remove current active page
    activateLink(this.pageListElt, route.page.key);

    // Remove current active page
    this.renderPageMenu(route);
    activateLink(this.menuListElt, route.menu.key);

    // Update Page content
    this.pageContainerElt.innerHTML = `<${route.menu.element}></${route.menu.element}>`;
  }
}
