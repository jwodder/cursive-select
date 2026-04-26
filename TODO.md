- These currently move the scrollbar but don't change the focus:
    - `w`, `Page Up` — go up a page
    - `z`, `Page Down` — go down a page
- Rename `*Selector` to `Select`? `SelectList`? `Selectable`?
- Let the user select nothing for a `RadioSelector`?

- Slight problem with scrolling: If you scroll to the bottom and then wrap
  around to the top, the first focusable item will be on top of the screen,
  cutting off the text for the first selection list.
- Problem: Long single-selection lines are cut off, while long multi-selection
  lines are wrapped
    - Report as bug?
- Inconsistency: When a single-selection option has the focus, the entire line
  is highlighted, but when a multi-selection option has the focus, only the
  checkbox is highlighted.

- After next release of `cursive` after v0.21.1:
    - Use `Checkbox::labelled()`
    - Use `MultiChoiceGroup`?
        - cf. <https://github.com/gyscos/cursive/blob/main/cursive/examples/checkbox_multichoicegroup.rs>

- Fill out README and put on GitHub?
    - Determine accurate MSRV
    - Rename to `cursive-select`?
    - Rename `Curselect` to `Form`?
