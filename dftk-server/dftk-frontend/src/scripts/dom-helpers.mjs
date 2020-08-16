// Menus

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
export const currentForm = (elt) => {
  if (elt === null || elt.tagName.toLowerCase() === "dftk-form") {
    return elt;
  }
  return currentForm(elt.parentElement) || null;
};
