import { ApplicationElt } from "./application/application.mjs";
import {
  GeneralElt,
  MemberTypesElt,
  RoomsElt,
  ScheduleElt,
  SessionFormatsElt,
  SessionsCategoriesElt,
  SessionsElt,
  SlotsElt,
  SpeakersElt,
  SponsorCategoriesElt,
  SponsorsElt,
  TeamElt,
  UsersElt,
} from "./pages/index.mjs";
import { Router } from "./fmwk/routes.mjs";
import { Store } from "./fmwk/store.mjs";
import { effect } from "./reducer.mjs";
import { pages } from "./routes.mjs";
import { defaultState } from "./state.mjs";
import { createLogger } from "./fmwk/logger.mjs";

// Register Web Components
customElements.define("dftk-app", ApplicationElt);

customElements.define("dftk-general", GeneralElt);
customElements.define("dftk-users", UsersElt);

customElements.define("dftk-sessions", SessionsElt);
customElements.define("dftk-session-categories", SessionsCategoriesElt);
customElements.define("dftk-session-formats", SessionFormatsElt);

customElements.define("dftk-speakers", SpeakersElt);

customElements.define("dftk-sponsors", SponsorsElt);
customElements.define("dftk-sponsor-categories", SponsorCategoriesElt);

customElements.define("dftk-team", TeamElt);
customElements.define("dftk-member-types", MemberTypesElt);

customElements.define("dftk-rooms", RoomsElt);
customElements.define("dftk-slots", SlotsElt);
customElements.define("dftk-schedule", ScheduleElt);

// Bootstrap application
const logger = createLogger("main");

const state = defaultState();
export const store = new Store(state);
export const router = new Router(store, pages);

const root = document.querySelector("dftk-app");
if (root) {
  root.connect(router);
  document.addEventListener("action", (event) =>
    effect(store, event.detail || {})
  );
} else {
  logger.error("No root application found!");
}
