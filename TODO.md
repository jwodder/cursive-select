- Improve spacing around buttons
- Test wrapping of long text

- Keybindings to add:
    - `g` — jump to top
    - `G` — jump to bottom
    - `j` — move down, wrapping around ?
    - `k` — move up, wrapping around ?
    - `w` — go up a page
    - `z` — go down a page
    - `Tab` — go to next selection list
    - `Shift`+`Tab` — go to previous selection list

- Stylize selection list titles?
- Add a cell of padding around the view area?
- `Selector::Single`: Handle `default` being out of range for `options`?

- Allow setting a list/set of defaults for multi-selections
- Better handling of calling `run()` with no selection lists?
- Get rid of the shadow on the view?

- Slight problem with scrolling: If you scroll to the bottom and then wrap
  around to the top, the first focusable item will be on top of the screen,
  cutting off the text for the first selection list.

- After next release of `cursive` after v0.21.1:
    - Use `Checkbox::labelled()`
    - Use `MultiChoiceGroup`?
        - cf. <https://github.com/gyscos/cursive/blob/main/cursive/examples/checkbox_multichoicegroup.rs>
