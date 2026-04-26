- These currently move the scrollbar but don't change the focus:
    - `w`, `Page Up` — go up a page
    - `z`, `Page Down` — go down a page
- Support activating buttons with `Space`

- `Selector::Single`: Handle `default` being out of range for `options`?
- Allow setting a list/set of defaults for multi-selections?

- Define separate `SingleSelector` and `MultiSelector` types that can be
  converted `Into` `Selector`
    - `SingleSelector` → `RadioSelector`?
    - Support using any `S: AsRef<str>` when constructing selectors
        - Or `Into<String>`?
    - Give selectors `new(title: AsRef<str>, options: IntoIterator<Item:
      AsRef<str>>) -> Self` constructors?
        - If a selector has no options, ignore/don't render it?
    - Give selectors methods for setting the default(s)
    - Give selectors methods for getting the defaults that take care of
      out-of-range values
        - For `SingleSelector`, an out-of-range default is replaced with zero
        - For `MultiSelector`, an out-of-range default is ignored

- Make the examples output both the indices and labels for the selected
  options?
    - Show the raw return value if a `--debug` option is given?

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
    - Rename to `cursive-select`?
    - Rename `Curselect` to `Form`?
