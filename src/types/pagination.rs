use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is getting extracted from query params.
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first item to be retuned.
    pub start: usize,
    /// The index of the last item to be retuned.
    pub end: usize,
}

/// Extract Pagination params from the given query params.
/// # Example query
/// GET `/questions?start=1&end=10` returns a Pagination { 1, 10 }.
/// # Example usage
/// ```rust
///  let query = HashMap::from(
///     [("start".to_string(), "1".to_string()),
///     ("end".to_string(), "10".to_string())
///  ];
///
///  let p = types::pagination::extract_pagination(query).unwrap();
///  assert_eq!(p.start, 1);
///  assert_eq!(p.end, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
