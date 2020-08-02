import { createLogger } from "./logger.mjs";

const logger = createLogger("Store");

export class Store {
  get state() {
    return this.eventualState;
  }

  constructor(initialState) {
    this.eventualState = Promise.resolve(initialState);
    this.listeners = new Map();
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
        (this.listeners.get(key) || []).forEach((cb) => cb(value));
      }
      return state;
    });
  }
}
