// Menus

export const createMenubar = (label) => {
  const elt = document.createElement("ul");
  elt.setAttribute("role", "menubar");
  elt.setAttribute("aria-label", label);

  return elt;
};

export const createMenuItem = (key, label, href) => {
  const elt = document.createElement("li");
  elt.classList.add(`key-${key}`);
  elt.setAttribute("role", "none");
  const lnk = document.createElement("a");
  lnk.href = href;
  lnk.innerText = label;
  lnk.setAttribute("menuitem", "");
  elt.appendChild(lnk);
  return elt;
};

export const activateLink = (parentElt, key) => {
  const currentPage = parentElt.querySelector(".active");
  if (currentPage) {
    currentPage.classList.remove("active");
  }
  // Apply new current active  page
  const newPage = parentElt.querySelector(`.key-${key}`);
  if (newPage) {
    newPage.classList.add("active");
  }
};


// Form
export const createFieldset = (legend) => {
  const elt = document.createElement('fieldset');
  if(legend) {
    const legendElt = document.createElement('legend');
    legendElt.textContent = legend;
    elt.appendChild(legendElt);
  }
  return elt;
}

export const createInputField = (name, label, type) => {
  const field = document.createElement('div');
  field.classList.add('field', `field-${name}`);
  const id = `fld-${name}`;

  if (label) {
    const labelElt = document.createElement('label');
    labelElt.htmlFor = id;
    labelElt.textContent = label
    field.appendChild(labelElt);
  }

  const inputElt = document.createElement('input');
  inputElt.type = type || 'text';
  inputElt.id = id;
  inputElt.name = name;
  field.appendChild(inputElt);

  return field;
};