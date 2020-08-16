import { ActionsElt } from "./form/actions.mjs";
import { ButtonElt } from "./form/button.mjs";
import { FieldElt } from "./form/field.mjs";
import { FieldsetElt } from "./form/fieldset.mjs";
import { MenubarElt } from "./menu/menubar.js";
import { SubmenuElt } from "./menu/submenu.js";

// Forms
customElements.define("dftk-button", ButtonElt);
customElements.define("dftk-field", FieldElt);
customElements.define("dftk-fieldset", FieldsetElt);
customElements.define("dftk-actions", ActionsElt);

// Menubar
customElements.define("dftk-menubar", MenubarElt);
customElements.define("dftk-submenu", SubmenuElt);
