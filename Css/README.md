# CSS Selector

1. tagName (h1,h2,.., div,..)
```html
  <h1>Hello World</h1>
  <h1>Welcome to our website</h1>
```
```css
  /* selects both <h1> elements */
  h1 {
    font-size: 32px;
  }
```

2. className (.class-name)
```html
  <div class='card'>
    Some content
  </div>
```
```css
  .card {
    min-height: 100vh;
  }
```

3. multiple classes (.class-name1.class-name2) 'No space'
```html
  <button class='btn btn--primary'>
    Button 1
  </button>
  <button class='btn btn--secondary'>
    Button 2
  </button>
  <button class='btn--secondary'>
    Button 3
  </button>
```
```css
  .btn {
    /* selects all three buttons */
  }

  .btn.btn--secondary {
    /* only selects the 2nd button */
  }

  .btn--secondary {
    /* select both 2nd and 3rd */
  }
```

4. idName (#id-name)
```html
  <!-- Note: id is unique -->
  <div id='card'>
    Some content
  </div>
```
```css
  #card {
    min-height: 100vh;
  }
```

# CSS Child Selector

- `<SPC>` -> descandent selector
- `>` -> direct child selector

```html
  <div class='card'>
    <h1>Title</h1>
    <div>
      <h1>Title inside a <span>div</span></h1>
    </div>
  </div>
```
```css
  .card h1 {
    /* selects both of the h1 tag */
  }

  .card > h1 {
    /* selects only the h1 with 'Title' */
  }

  .card div h1 {
    /* selects the h1 with 'Title inside a <span>div</span>' */
  }
```

# CSS Sibling Selector

- `+` -> next sibling (comes just after)
- `~` -> comes after sibling

```html
<ul>
  <li>list item 1</li>
  <li id='main'>list item 2</li>
  <li>list item 3</li>
  <li>list item 4</li>
  <li>list item 5</li>
</ul>
```
```css
  #main + li {
    /* select only the 3rd li element */
  }

  #main ~ li {
    /* select all li element after 2nd li (3,4,5) */
  }
```

# CSS Other Selector

- `[attr]` -> attribute selector

```html
<form>
  <input type='text'>
  <input type='number'>
</form>
```
```css
  input[type='text'] {
    /* select only the input element with the attribute type='text' */
  }
```

# CSS units

## absolute

  This is absolute meanings, it's not depend are others
  - px (majority)
  - pt
  - pc
  - in

## relative

  This units depend on some other property for it's value.
  1. rem

  This units is a fraction of the `font-size` set in `html` element
  __NOTE__: by default it's `16px`

  generally, use with font-size (!important)

  ```css
  h1 {
    font-size: 2rem; /* 2 x 16px = 32px */
  }
  ```

  2. em

  This units is a fraction of the `font-size` of the element itself
  __NOTE__: if used in `font-size` itself then look on the parent element's font-size

  ```css
  .parent {
    font-size: 1rem;
  }

  .child {
    font-size: .5em; /* parent's font-size: 16px -> 0.5 * 16px = 8px */
    margin: 1.5em; /* 1.5 x 8px = 12px */
  }
  ```

  3. %

  This units is percentage of the total avialable value.
  general use in `box-modal property` and `display property`.

  4. vw vh

  This unit is a fraction of the viewport width `vw` and viewport height `vh`
  say, viewport is 1440 X 800

  ```css
  div {
    width: 50vw; /* 720px */
    min-height: 75vh; /* 600px */
  }
  ```

# CSS Display

  There are three types of displayed element
  1. block
  2. inline
  3. inline-block

  ## block element

  - nth number of possible child node (including a TextNode)
  - Can have all three box-modal property
  - The Next sibling will come below (always)
  - Here, you *can* set `width` and `height` and use the box-sizing property

  ## inline element

  - TextNode and `inline` element are the only possible child nodes
  - Can have all three box-modal property
  - The Next sibling will come to the right if space is aviable
  - Here, you *cannot* set `width` and `height` and use the box-sizing property

  ## inline-block element

  The main difference between `inline` and `inline-block` is that
  we can set the `width` and `height` property.

# CSS Position

  ## Static element

  - This is the default
  - Element are placed according to the flow of the markup

  ## Positioned element

  An element is said to be *positioned element* if it's `position` property isn't set to `static`
  - On setting, element come on-top of the layout. (z-index increased)
  - Eventhough, it's `position property` changed it's position in screen remains unchanged

  ## relative & absolute

  - relative
    In case of `position: relative` the element is positioning is done relative to it's static position.  

    in fact, other element treat it as a `static` element.

  - absolute
    Here, the positioning is done w.r.t to the `nearest positioned ancestor`.

  __NOTE__:  
    * element is totally out of the regular flow.
    * next sibling will take it's place.

# CSS Property Reference

## Typography property

  - font-family
  - font-size
  - font-weight

  - line-height
  - letter-spacing
  - word-spacing
  - white-space

  - text-align
  - text-indent
  - text-shadow
  - text-transform

## Color property

  - color
  - background-color
  - background-image
  - background-repeat
  - background-size
  - background-position
  - box-shadow

## Box modal property

  - width
  - height
  - min-[width, height], max-[width, height]
  - box-sizing

  - padding
  - padding-[inline, block]

  - border
  - border-radius
  - outline
  - outline-radius
  - outline-offset

  - margin
  - margin-[inline, block]

## Display property

  - display
  - position
  - top, bottom, left, right
  - z-index

## Flex & Grid

  - gap
  - align-items
  - align-content
  - justify-items
  - justify-content

  - align-self
  - justify-self

  ### Flex

  - flex-flow
  - flex-direction

  - flex-basis
  - flex-grow
  - flex-shrink

  ### Grid

  - grid-template-rows
  - grid-template-columns
  - grid-template-areas
  - grid-auto-flow
  - grid-auto-rows
  - grid-auto-columns

  - grid-row
  - grid-column
  - grid-area

## Animation Property

  - transform
  - translate
  - animation
  - @keyframes

## Pseudso Element & Classes

  - cursor
  - caret-color
  - content