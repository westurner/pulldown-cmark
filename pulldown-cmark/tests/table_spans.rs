use pulldown_cmark::{Event, Options, Parser, Tag};

#[test]
fn table_spans_are_opt_in_and_exposed_on_cell_events() {
    let source = "| a {colspan=2 rowspan=3} | b |\n|---|---|\n| c | d |\n";

    let default_cells: Vec<_> = Parser::new_ext(source, Options::ENABLE_TABLES)
        .filter_map(|event| match event {
            Event::Start(Tag::TableCell { colspan, rowspan }) => Some((colspan, rowspan)),
            _ => None,
        })
        .collect();
    assert!(default_cells
        .iter()
        .all(|&(colspan, rowspan)| (colspan, rowspan) == (1, 1)));

    let mut options = Options::ENABLE_TABLES;
    options.insert(Options::ENABLE_TABLE_SPANS);
    let cells: Vec<_> = Parser::new_ext(source, options)
        .filter_map(|event| match event {
            Event::Start(Tag::TableCell { colspan, rowspan }) => Some((colspan, rowspan)),
            _ => None,
        })
        .collect();
    assert_eq!(cells[0], (2, 3));
    assert_eq!(cells[1], (1, 1));
}

#[test]
fn table_spans_render_to_html_attributes() {
    let source = "| a {colspan=2 rowspan=2} | b |\n|---|---|\n| c | d |\n";
    let mut options = Options::ENABLE_TABLES;
    options.insert(Options::ENABLE_TABLE_SPANS);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, Parser::new_ext(source, options));
    assert!(html.contains("colspan=\"2\""));
    assert!(html.contains("rowspan=\"2\""));
    assert!(!html.contains("{colspan=2"));
}

#[test]
fn malformed_table_span_attributes_are_ignored() {
    let source = "| a {colspan=zero} | b |\n|---|---|\n| c | d |\n";
    let mut options = Options::ENABLE_TABLES;
    options.insert(Options::ENABLE_TABLE_SPANS);
    let cells: Vec<_> = Parser::new_ext(source, options)
        .filter_map(|event| match event {
            Event::Start(Tag::TableCell { colspan, rowspan }) => Some((colspan, rowspan)),
            _ => None,
        })
        .collect();
    assert!(cells
        .iter()
        .all(|&(colspan, rowspan)| (colspan, rowspan) == (1, 1)));
}

#[test]
fn zero_colspan_is_preserved_as_an_explicit_extension_value() {
    let source = "| a {colspan=0} | b |\n|---|---|\n| c | d |\n";
    let mut options = Options::ENABLE_TABLES;
    options.insert(Options::ENABLE_TABLE_SPANS);
    let cells: Vec<_> = Parser::new_ext(source, options)
        .filter_map(|event| match event {
            Event::Start(Tag::TableCell { colspan, rowspan }) => Some((colspan, rowspan)),
            _ => None,
        })
        .collect();
    assert_eq!(cells[0], (0, 1));

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, Parser::new_ext(source, options));
    assert!(html.contains("colspan=\"0\""));
}

#[test]
fn dom_attributes_are_not_part_of_table_span_syntax() {
    let source = "| a {id=hook class=hostile data-x=1} | b |\n|---|---|\n| c | d |\n";
    let mut options = Options::ENABLE_TABLES;
    options.insert(Options::ENABLE_TABLE_SPANS);
    let cells: Vec<_> = Parser::new_ext(source, options)
        .filter_map(|event| match event {
            Event::Start(Tag::TableCell { colspan, rowspan }) => Some((colspan, rowspan)),
            _ => None,
        })
        .collect();
    assert!(cells
        .iter()
        .all(|&(colspan, rowspan)| (colspan, rowspan) == (1, 1)));

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, Parser::new_ext(source, options));
    assert!(!html.contains("id=\"hook\""));
    assert!(!html.contains("class=\"hostile\""));
    assert!(!html.contains("data-x=\"1\""));
}
