import {createLogger} from "../../fmwk/logger.mjs";

const CSS = ``;

export class SessionFormatsElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("SessionFormatsElt");
  }

  // Lifecycle
  connectedCallback() {
    this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });
    this.renderInit();
    this.renderState();
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
    h2.textContent = "Session Formats";
    headerElt.appendChild(h2);

    // Construction
    this.shadowRoot.appendChild(styleElt);
    this.shadowRoot.appendChild(headerElt);
  }

  renderState() {
    this.logger.debug(`renderState`);
  }
}
