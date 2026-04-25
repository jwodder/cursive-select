- Keybindings to add:
    - These currently move the scrollbar but don't change the focus:
        - `g`, `Home` — jump to top
        - `G`, `End` — jump to bottom
        - `w`, `Page Up` — go up a page
        - `z`, `Page Down` — go down a page
    - `Tab` — go to next selection list
    - `Shift`+`Tab` — go to previous selection list
- Stylize selection list titles?

- `Selector::Single`: Handle `default` being out of range for `options`?
- Allow setting a list/set of defaults for multi-selections
- Better handling of calling `run()` with no selection lists?

- Slight problem with scrolling: If you scroll to the bottom and then wrap
  around to the top, the first focusable item will be on top of the screen,
  cutting off the text for the first selection list.
    - Report as bug?
- Problem: Long single-selection lines are cut off, while long multi-selection
  lines are wrapped
    - Report as bug?
- Inconsistency: When a single-selection option has the focus, the entire line
  is highlighted, but when a multi-selection option has the focus, only the
  checkbox is highlighted.
- Problem: If `CircularFocus` is used inside a `Dialog`, the buttons will only
  be selectable via the mouse — but without `CircularFocus`, arrow keys don't
  wrap around (though tab & shift-tab still do).

- After next release of `cursive` after v0.21.1:
    - Use `Checkbox::labelled()`
    - Use `MultiChoiceGroup`?
        - cf. <https://github.com/gyscos/cursive/blob/main/cursive/examples/checkbox_multichoicegroup.rs>
