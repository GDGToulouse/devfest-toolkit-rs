import { createLogger } from "../fmwk/logger.mjs";
import { activeRoute$ } from "../state.mjs";

const CSS = `
:host {
  border-radius: .25em;
  margin: .25em .25em .25em 0;
  padding: .25em;
  background-color: var(--app-content-bg);
  overflow: auto;
  display: flex;
}

main {
  flex: 1 1 auto;
}
`;

export class CurrentRouteElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("CurrentRouteElt");
    this.pageElt = document.createElement("main");
  }

  // Lifecycle
  connectedCallback() {
    // this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });

    const styleElt = document.createElement("style");
    styleElt.innerHTML = CSS;

    this.shadowRoot.append(styleElt, this.pageElt);

    // Register events
    document.addEventListener("store", (evt) =>
      evt.detail.register(activeRoute$, (route) => this.updateRoute(route))
    );
  }

  updateRoute(route) {
    this.pageElt.innerHTML = route ? route.body : "";
  }
}
