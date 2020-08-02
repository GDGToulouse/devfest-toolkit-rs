import { createAction, queryAction } from "../../fmwk/actions.mjs";
import { createLogger } from "../../fmwk/logger.mjs";
import { lens } from "../../fmwk/optics.mjs";

const CSS = ``;

const siteInfo$ = lens("site.info");

const query = `query {
  info {
    id
    name
    address {
      locality { shortName longName}
      country { shortName longName}
      latLng { lat lng}
    }
    languages {main others}
    dates { start end}
  }
}`;

export class GeneralElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("GeneralElt");

    // Elements
    this.formElt = document.createElement("form");
  }

  // Lifecycle
  connectedCallback() {
    this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });
    this.renderInit();
  }

  // Render
  renderInit() {
    this.logger.debug(`renderInit`);

    // Style
    const styleElt = document.createElement("style");
    styleElt.innerHTML = CSS;

    // Header
    const headerElt = document.createElement("header");
    const h2 = document.createElement("h2");
    h2.textContent = "General";
    headerElt.appendChild(h2);

    // Form
    // FIXME

    // Construction
    this.shadowRoot.appendChild(styleElt);
    this.shadowRoot.appendChild(headerElt);
    this.shadowRoot.appendChild(this.formElt);

    // Register inner events
    this.formElt.addEventListener("submit", (event) => this.save(event));

    // Register outer events
    const app = document.querySelector("dftk-app");
    if (app && app.store) {
      this.logger.debug("handle site info");
      app.store.register(siteInfo$, (info) => this.updateInfo(info));
      const event = createAction(queryAction(siteInfo$, query, "info"));
      app.dispatchEvent(event);
    }
  }

  // Outer events
  updateInfo(info) {
    this.formElt.innerHTML = `<pre>${JSON.stringify(info, null, 2)}</pre>`;
  }

  // Inner events
  save(event) {
    this.logger.info("Save", { event });
    event.stopImmediatePropagation();
    return false;
  }
}
