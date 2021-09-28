pub enum PathSegment<'a> {
    Literal(&'a str),
    Parameter {
        name: &'a str,
        regex: Option<&'a str>,
    },
}

pub fn parse(path: &str) -> impl Iterator<Item = PathSegment<'_>> {
    path.split('/')
        // skip the leading empty segment
        .skip(1)
        .map(
            |segment| match segment.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                Some(segment) => {
                    let mut it = segment.splitn(2, ':');
                    PathSegment::Parameter {
                        name: it.next().unwrap(),
                        regex: it.next(),
                    }
                }
                None => PathSegment::Literal(segment),
            },
        )
}
