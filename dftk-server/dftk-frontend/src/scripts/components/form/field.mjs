import { createLogger } from "../../fmwk/index.mjs";

export class FieldElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("FieldElt");
    // Element
    this.labelElt = null;
    // Templates
    this.field = document.getElementById("field");
  }

  // Attributes
  static get observedAttributes() {
    return ["id", "label"];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    // this.logger.debug(name, 'change', oldValue, '->', newValue);

    if (name === "label" && this.labelElt) {
      this.labelElt.textContent = newValue;
    }
    if (name === "id" && this.labelElt) {
      this.labelElt.htmlFor = newValue;
    }
  }

  // Lifecycle
  connectedCallback() {
    // this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });

    const fieldContent = this.field.content.cloneNode(true);
    this.shadowRoot.appendChild(fieldContent);

    this.labelElt = this.shadowRoot.querySelector("label");
    const id = this.getAttribute("id");
    if (id) {
      this.labelElt.htmlFor = id;
    }
    const label = this.getAttribute("label");
    if (label) {
      this.labelElt.textContent = label;
    }
  }
}
