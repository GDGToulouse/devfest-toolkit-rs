import {createFieldset, createInputField} from "../../dom-helpers.mjs";
import {createAction, queryAction} from "../../fmwk/actions.mjs";
import {createLogger} from "../../fmwk/logger.mjs";
import {lens} from "../../fmwk/optics.mjs";
import {FORM_CSS} from "../form.js";

const CSS = `
${FORM_CSS}

`;

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
    const idElt = createInputField('id', 'Id');
    const nameElt = createInputField('name', 'Name');

    const addressElt = createFieldset('Address');
    // FIXME

    const languagesElt = createFieldset('Languages');
    // FIXME

    const datesElt = createFieldset('Dates');
    // FIXME

    this.formElt.appendChild(idElt);
    this.formElt.appendChild(nameElt);
    this.formElt.appendChild(addressElt);
    this.formElt.appendChild(languagesElt);
    this.formElt.appendChild(datesElt);

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
    // FIXME
    const {id, name, address, languages, dates} = info;
    this.formElt['id'].value = id;
    this.formElt['name'].value = name;
  }

  // Inner events
  save(event) {
    this.logger.info("Save", { event });
    event.stopImmediatePropagation();
    return false;
  }
}
