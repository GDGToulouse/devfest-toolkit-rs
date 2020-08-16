import { createLogger } from "../../fmwk/index.mjs";

export class ActionsElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("ActionsElt");
    // Templates
    this.actionsElt = document.getElementById("actions");
  }

  // Lifecycle
  connectedCallback() {
    // this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });

    const content = this.actionsElt.content.cloneNode(true);
    this.shadowRoot.appendChild(content);
  }
}
