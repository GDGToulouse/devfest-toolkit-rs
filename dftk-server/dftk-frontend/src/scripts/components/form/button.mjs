import {
  generalAction,
  createAction,
  createLogger,
  lens,
} from "../../fmwk/index.mjs";

const CSS = `
button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  align-self: start; // Optional - see "Gotchas"
  
  box-sizing: border-box;
  border: medium solid rgba(0,0,0,.25);
  background-color: transparent;

  font-size: 1rem;
  font-family: inherit;
  padding: 0;
  cursor: pointer;
  
  background-color: var(--btn-bg, #999);
  color: var(--btn-fg, #222);
  border-radius: .25em;
  box-shadow: 0 3px 5px rgba(0, 0, 0, 0.18);

  padding: 0.125em 0.5em;
  min-width: 16ch;
  min-height: 44px;

  text-align: center;
  line-height: 1.0;

  transition: 220ms all ease-in-out;
}

button:hover, button:active {
  background-color: var(--btn-bg-light, #bbb);
}
button:focus {
    outline-style: solid;
    outline-color: transparent;
    box-shadow: 0 0 0 4px var(--btn-shadow, rgba(0,0,0,.4));
}
button:disabled {
 filter: grayscale(1);
}

@media screen and (-ms-high-contrast: active) {
  button {
    border: 2px solid currentcolor;
  }
}
`;

const counter = {};
const newActionId = (action) => {
  const count = counter[action] || 0;
  counter[action] = count + 1;
  return counter[action];
};

export class ButtonElt extends HTMLElement {
  constructor() {
    super();
    this.logger = createLogger("ButtonElt");
    this.label = null;
    this.icon = null;
    this.action = "nope";
    this.actionId = null;
    // Elements
    this.buttonElt = document.createElement("button");
    this.buttonElt.type = "button";
  }

  // Attributes
  static get observedAttributes() {
    return ["label", "icon", "action"];
  }

  attributeChangedCallback(name, oldValue, newValue) {
    // this.logger.debug(name, 'change', oldValue, '->', newValue);
    this[name] = newValue;

    if (name === "icon") {
      const classes = Array.from(this.buttonElt.values()).filter((it) =>
        it.startsWith("icon-")
      );
      this.buttonElt.classList.remove(...classes);
      if (this.icon) {
        this.buttonElt.add(`icon-${this.icon}`);
      }
    }

    if (name === "label") {
      this.buttonElt.innerHTML = this.label;
    }
  }

  // Lifecycle
  connectedCallback() {
    // this.logger.debug(`connected`);
    this.attachShadow({ mode: "open" });

    const styleElt = document.createElement("style");
    styleElt.innerHTML = CSS;

    this.shadowRoot.append(styleElt, this.buttonElt);

    this.buttonElt.innerHTML = this.label || "Button";
    if (this.icon) {
      this.buttonElt.classList.add(`icon-${this.icon}`);
    }

    // Register events
    if (document.store && this.action) {
      this.buttonElt.addEventListener("click", () =>
        this.onButtonClick(document.store)
      );
      this.registerOnChange(document.store);
    }
  }

  onButtonClick(store) {
    this.logger.debug("Clicked", this.action);
    if (this.actionId !== null) {
      this.logger.warn(
        `Previous action ${this.actionId} not finished, we cancel the action!`
      );
      return;
    }
    this.buttonElt.disabled = true;
    this.actionId = newActionId(this.action);
    this.buttonElt.classList.add("icon-loading");
    // send action with id
    const id = newActionId(this.action);
    const action = createAction(generalAction(id, this.action));
    this.dispatchEvent(action);
  }

  registerOnChange(store) {
    const actionInfo$ = lens(`actions.${this.action}`);
    store.register(actionInfo$, (id) => {
      if (this.id <= id) {
        this.buttonElt.classList.remove("icon-loading");
        this.buttonElt.disabled = false;
      }
    });
  }
}
