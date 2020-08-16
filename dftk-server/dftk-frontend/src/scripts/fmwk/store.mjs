import { store } from "../index.mjs";
import { createLogger } from "./logger.mjs";
import { lens } from "./optics.mjs";

const logger = createLogger("Store");

export class Store {
  get state() {
    return this.eventualState;
  }

  constructor(initialState) {
    this.eventualState = Promise.resolve(initialState);
    this.listeners = new Map();
  }

  connect(elt, reducer) {
    elt.store = store;
    elt.addEventListener("action", (event) =>
      reducer(this, event.detail || {})
    );
    const event = new CustomEvent("store", { detail: this });
    elt.dispatchEvent(event);
  }

  register(selector, listener) {
    const { key } = selector;
    if (this.listeners.has(key)) {
      const array = this.listeners.get(key) || [];
      this.listeners.set(key, [...array, listener]);
    } else {
      this.listeners.set(key, [listener]);
    }
  }

  unregister(selector, listener) {
    const { key } = selector;
    if (this.listeners.has(key)) {
      const array = this.listeners.get(key) || [];
      this.listeners.set(
        key,
        array.filter((it) => it !== listener)
      );
    }
  }

  updateState(selector, value) {
    const { key } = selector;
    logger.debug("updateState", { key, value });
    this.eventualState = this.state.then((state) => {
      const updated = selector.set(state, value);
      if (updated) {
        logger.info("updated", state);
        for (let entry of this.listeners.entries()) {
          const [path, cbs] = entry;
          if (path === key) {
            cbs.forEach((cb) => cb(value));
          } else if (path.startsWith(key)) {
            const x = lens(path).get(state);
            cbs.forEach((cb) => cb(x));
          }
        }
        (this.listeners.get(key) || []).forEach((cb) => cb(value));
      }
      return state;
    });
  }
}
