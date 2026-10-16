# Changelog

## Unreleased

- Add opt-in table cell span parsing with `Options::ENABLE_TABLE_SPANS`.
  `{colspan=N rowspan=M}` attributes are exposed on `Tag::TableCell` and
  rendered by the HTML writer.
- Document the security boundary against full `markdown-it-attrs`-style
  arbitrary `id`, `class`, style, data, and event-handler attributes.