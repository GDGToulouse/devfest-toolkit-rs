# Ideas

## HTML

- Prefer HTML/CSS over JS
- Do not care of support of previous browser
- Web Component without any lib/tool/compiler
- try to be accessible, mostly by using standard
- design for desktop first

## CSS

- Do not care of support of previous browser
- Theme based on CSS custom properties
- Use CSS, use a bundler for aggregation
- use <https://necolas.github.io/normalize.css/> Normalizer

- use a SVG icon system similar to <https://github.com/GDGToulouse/devfest-theme-hugo/blob/c1a0f67a4dd287623d5f87efcf0260211fd0bba3/assets.js>

Inspiration:
[Saagie Design System](https://7-design-system.public.prod.saagie.io/v/0.38.2/)
[Clever Cloud WebComponent](https://www.clever-cloud.com/doc/clever-components/)
[Colors](http://clrs.cc/)
[1-line layouts](https://1linelayouts.glitch.me/) 

## Javascript

- Use standard lib
- No framework
- Avoid lib

- Try ParcelJS 2.x Bundler (parcel@next) ?
- Try storybook for WebComponent ?
  <https://github.com/storybookjs/storybook>
  <https://www.npmjs.com/package/@storybook/web-components>
  
- Implement a UDF, try optics

### Routing

- url = f(state) 
- state = f(url)

url = /{page}

Page => one query 

State only changed with sending JS event 'action'

See Nav: <https://css-tricks.com/three-css-alternatives-to-javascript-navigation/>

### Testing 

- See it later, prefer Jest

### Maybe replace tooling (Yak shaving mode)

- bundling code (rollup, babel, swc? ...)
- need to serve page
  - with watching change
  - with proxying some routes