import { createLogger } from "../../fmwk/logger.mjs";

const CSS = ``;

export class MemberTypesElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("MemberTypesElt");
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
    h2.textContent = "Member Types";
    headerElt.appendChild(h2);

    // Construction
    this.shadowRoot.append(styleElt, headerElt);
  }

  renderState() {
    this.logger.debug(`renderState`);
  }
}
