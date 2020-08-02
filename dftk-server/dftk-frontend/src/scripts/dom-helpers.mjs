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
