use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is getting extracted from query params.
#[derive(Debug, Default)]
pub struct Pagination {
    /// The index of the last item to be retuned.
    pub limit: Option<i32>,
    /// The index of the first item to be retuned.
    pub offset: i32,
}

/// Extract Pagination params from the given query params.
/// # Example query
/// GET `/questions?start=1&end=10` returns a Pagination { 1, 10 }.
/// # Example usage
/// ```rust
///  let query = HashMap::from(
///     [("limit".to_string(), "1".to_string()),
///     ("offset".to_string(), "10".to_string())
///  ];
///
///  let p = types::pagination::extract_pagination(query).unwrap();
///  assert_eq!(p.limit, Some(1));
///  assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::ParseError)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}
