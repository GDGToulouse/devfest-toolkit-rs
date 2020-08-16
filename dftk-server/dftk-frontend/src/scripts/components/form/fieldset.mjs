import { createLogger } from "../../fmwk/index.mjs";

export class FieldsetElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("FieldsetElt");
    // Element
    this.legendElt = null;
    // Templates
    this.fieldset = document.getElementById("fieldset");
  }

  // Attributes
  static get observedAttributes() {
    return ["label"];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    this.logger.debug(name, "change", oldValue, "->", newValue);

    if (name === "label" && this.labelElt) {
      this.legendElt.textContent = newValue;
    }
  }

  // Lifecycle
  connectedCallback() {
    // this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });

    const fieldsetContent = this.fieldset.content.cloneNode(true);
    this.shadowRoot.appendChild(fieldsetContent);

    this.legendElt = this.shadowRoot.querySelector("legend");
    const label = this.getAttribute("label");
    if (label) {
      this.legendElt.textContent = label;
    }
  }
}
