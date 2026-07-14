use super::*;

/*
0 1 2 3
w a r p
-------
0     4  << the span for the string "octomus" is (0, 4)

Spanned {
    item: String::new("octomus"),  << octomus string
    span: Span::new(0, 4)       << span
}

or >> String::new("octomus").spanned(Span::new(0, 4))        */
fn octomus() -> Spanned<String> {
    String::from("octomus").spanned(Span::new(0, 4))
}

fn empty() -> Spanned<String> {
    String::new().spanned_unknown()
}

#[test]
fn knows_distances() {
    assert!(octomus().span.distance() == 4);
    assert!(empty().span.distance() == 0);
}
