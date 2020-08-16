import { createLogger } from "../fmwk/logger.mjs";

export class RouteElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("RouteElt");
    this.label = null;
    this.menu = null;
    this.body = this.innerHTML;
  }

  // Attributes
  static get observedAttributes() {
    return ["label", "menu"];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    this[name] = newValue;
    this.dispatchRoute();
  }

  // Lifecycle
  connectedCallback() {
    // this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });
  }

  dispatchRoute() {
    const { label, menu, body } = this;
    if (label !== null && menu !== null) {
      const event = new CustomEvent("route", {
        bubbles: true,
        composed: true,
        detail: { label, menu, body },
      });
      this.dispatchEvent(event);
    }
  }
}
