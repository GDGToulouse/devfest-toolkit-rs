import { GeneralElt } from "./general/general.mjs";
import { UsersElt } from "./general/users.mjs";
import { RoomsElt } from "./schedule/rooms.mjs";
import { ScheduleElt } from "./schedule/schedule.mjs";
import { SlotsElt } from "./schedule/slots.mjs";
import { SessionsCategoriesElt } from "./sessions/categories.mjs";
import { SessionFormatsElt } from "./sessions/formats.mjs";
import { SessionsElt } from "./sessions/sessions.mjs";
import { SpeakersElt } from "./speakers/speakers.mjs";
import { SponsorCategoriesElt } from "./sponsors/categories.mjs";
import { SponsorsElt } from "./sponsors/sponsors.mjs";
import { MemberTypesElt } from "./team/member-types.mjs";
import { TeamElt } from "./team/team.mjs";

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
