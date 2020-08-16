import {
  createAction,
  queryAction,
  createLogger,
  lens,
} from "../../fmwk/index.mjs";

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
    this.formElt = null;
    // Templates
    this.general = document.getElementById("general");
  }

  // Lifecycle
  connectedCallback() {
    // this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });
    this.renderInit();
  }

  // Render
  renderInit() {
    // this.logger.debug(`renderInit`);
    const styleElt = document.createElement("style");
    styleElt.innerHTML = CSS;
    const content = this.general.content.cloneNode(true);
    this.shadowRoot.append(styleElt, content);
    this.formElt = this.shadowRoot.querySelector("form");

    // Register outer events
    if (document.store) {
      document.store.register(siteInfo$, (info) => this.updateInfo(info));
      this.triggerQueryAction(document.store);
    }
    this.formElt.addEventListener("submit", (event) => {
      event.preventDefault();
      this.save();
      return false;
    });
  }

  triggerQueryAction(store) {
    const event = createAction(queryAction(siteInfo$, query, "info"));
    this.dispatchEvent(event);
  }

  // Outer events
  updateInfo(info) {
    this.logger.info("Update General", info);
    const { id, name, address, languages, dates } = info;
    this.formElt["id"].value = id;
    this.formElt["name"].value = name;
    this.formElt["address.locality.shortName"].value =
      address.locality.shortName;
    this.formElt["address.locality.longName"].value =
      address.locality.shortName;
    this.formElt["address.country.shortName"].value = address.country.shortName;
    this.formElt["address.country.longName"].value = address.country.shortName;
    this.formElt["address.latLng.lat"].value = address.latLng.lat;
    this.formElt["address.latLng.lng"].value = address.latLng.lng;
    this.formElt["languages.main"].value = languages.main;
    this.formElt["dates.start"].value = dates.start;
    this.formElt["dates.end"].value = dates.end;

    Array.from(
      this.shadowRoot.querySelectorAll("[name='languages.others']")
    ).forEach((elt) => {
      this.checked = languages.others.includes(elt.value);
    });
  }

  // Inner events
  save() {
    const others = Array.from(
      this.shadowRoot.querySelectorAll("[name='languages.others']:checked")
    ).map((it) => it.value);

    const data = {
      id: this.formElt["id"].value,
      name: this.formElt["name"].value,
      address: {
        locality: {
          shortName: this.formElt["address.locality.shortName"].value,
          longName: this.formElt["address.locality.longName"].value,
        },
        country: {
          shortName: this.formElt["address.country.shortName"].value,
          longName: this.formElt["address.country.longName"].value,
        },
        latLng: {
          lat: this.formElt["address.latLng.lat"].value,
          lng: this.formElt["address.latLng.lng"].value,
        },
      },
      languages: {
        main: this.formElt["languages.main"].value,
        others,
      },
      dates: {
        start: this.formElt["dates.start"].value,
        end: this.formElt["dates.end"].value,
      },
    };
    this.logger.info("Save", { data });
    // FIXME
  }
}
