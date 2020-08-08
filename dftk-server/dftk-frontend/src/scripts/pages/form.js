export const FORM_CSS = `
form { 
  margin: 1em;
  display: flex;
  flex-direction: column; 
}
fieldset { 
  border: thin solid var(--darken);
  border-radius: .25em;
  display: flex;
  flex-direction: column; 
}
fieldset legend {
    margin-top: 1em;
    margin-left: -1em;
    font-size: 110%;
    font-weight: bold; 
}

form .field {
  margin-top: .5em;
}
form .field > label {
  font-weight: bold;
}
form .field > label::after {
  content: ':';
  padding-right: .5em;
}

form .field > input {
  padding: .5em .6em;
  border: 1px solid #ccc;
  box-shadow: inset 0 1px 3px #ddd;
  border-radius: .25em;
  vertical-align: middle;
  background-color: hsla(0, 100%, 100%, .8);
}

`;
