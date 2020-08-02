import { createLogger } from "./logger.mjs";

const logger = createLogger("Lens");

class AttributeLens {
  constructor(key) {
    this.key = key;
    this.guard = (state, block) => {
      const isArrayAccessor =
        Array.isArray(state) && typeof this.key === "number";
      const isObjectAccessor =
        typeof state === "object" && typeof this.key === "string";
      if (isArrayAccessor || isObjectAccessor) {
        return block(state);
      } else {
        throw new Error(
          `attribute '${this.key}' is not working with object '${state}'`
        );
      }
    };
  }

  get(state) {
    return this.guard(state, (state) => state[this.key]);
  }

  touch(state) {
    return this.guard(state, (state) => {
      const result = state[this.key];
      if (typeof result !== "undefined") {
        return result;
      }
      const touched = {};
      state[this.key] = touched;
      return touched;
    });
  }

  set(state, value) {
    return this.guard(state, (state) => {
      const oldValue = state[this.key];
      if (oldValue !== value) {
        state[this.key] = value;
        return true;
      } else {
        return false;
      }
    });
  }
}

class SelectByKeyLens {
  constructor(key) {
    this.key = key;
    this.predicate = (elt) => elt.key === this.key;
    this.guard = (state, block) => {
      if (Array.isArray(state)) {
        return block(state);
      } else {
        throw new Error(
          `object '${state}' cannot allow select by key '${this.key}'`
        );
      }
    };
  }

  get(state) {
    return this.guard(state, (state) => state.find(this.predicate));
  }

  touch(state) {
    return this.guard(state, (state) => {
      const result = state.find(this.predicate);
      if (typeof result !== "undefined") {
        return result;
      }
      const touched = { key: this.key };
      state.push(touched);
      return touched;
    });
  }

  set(state, value) {
    return this.guard(state, (state) => {
      const index = state.indexOf(this.predicate);
      if (index >= 0) {
        const oldValue = state[index];
        if (oldValue !== value) {
          state.splice(index, 1, value);
          return true;
        } else {
          return false;
        }
      } else {
        logger.info(
          `No element with key '${this.key}' found into '${state}', just push`
        );
        state.push(value);
        return true;
      }
    });
  }
}

const selectByKeyRE = /^\[(.*)\]$/;

class Lens {
  constructor(selector) {
    this.key = selector;
    // Parsing selector
    this.path = selector.split(".").map((str) => {
      const match = str.match(selectByKeyRE);
      if (match) {
        const key = match[1];
        return new SelectByKeyLens(key);
      } else {
        return new AttributeLens(str);
      }
    });
  }

  get(state) {
    return this.path.reduce((acc, elt) => elt.get(acc), state);
  }

  set(state, value) {
    if (this.path.length) {
      const parentPath = this.path.slice(0, this.path.length - 1);
      const parent = parentPath.reduce((acc, elt) => elt.touch(acc), state);
      const last = this.path[this.path.length - 1];
      return last.set(parent, value);
    } else {
      logger.warn("Identity does not change");
      return false;
    }
  }
}

export const lens = (selector) => {
  if (typeof selector !== "string") {
    throw new Error(`Expected a string, got ${selector}`);
  }
  return new Lens(selector);
};
