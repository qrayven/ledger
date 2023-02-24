use anyhow::{anyhow, bail, Context};
use itertools::Itertools;

type Id = usize;
type Timestamp = u32;

#[derive(Debug, PartialEq, Default, PartialOrd)]
pub struct Vertex {
    pub left: Option<Id>,
    pub right: Option<Id>,
    pub timestamp: Timestamp,
}

impl Vertex {
    pub fn from_str(str: impl AsRef<str>, id: usize) -> Result<Vertex, anyhow::Error> {
        let value_str = str.as_ref();
        let chunks: Vec<&str> = value_str
            .split_ascii_whitespace()
            .enumerate()
            .map(|(i, chunk)| {
                if i == 3 {
                    Err(anyhow!("the row has too many items: '{value_str}'"))
                } else {
                    Ok(chunk)
                }
            })
            .try_collect()?;

        if chunks.len() != 3 {
            bail!("the row has too few items: '{value_str}'")
        }

        let left_id: Id = chunks[0].parse().context("unable to parse the left ID")?;
        let right_id: Id = chunks[1].parse().context("unable to parse the right ID")?;
        let timestamp: Timestamp = chunks[2].parse().context("unable to parse the timestamp")?;

        // if node is self-referenced, the edge doesn't exist
        let left = if left_id == id { None } else { Some(left_id) };
        let right = if right_id == id { None } else { Some(right_id) };

        Ok(Vertex {
            left,
            right,
            timestamp,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_from_string_success() {
        let input = "0 1 2";

        let result = Vertex::from_str(input, 1).expect("shouldn't fail");
        assert_eq!(
            Vertex {
                left: Some(0),
                right: None,
                timestamp: 2
            },
            result
        );
    }

    #[test]
    fn try_from_sting_negative_integer() {
        let input = "0 1 -2";

        let err = Vertex::from_str(input, 1).expect_err("parsing error");
        assert!(
            err.to_string().contains("unable to parse the timestamp"),
            "{err}"
        )
    }

    #[test]
    fn try_from_string_not_enough_items() {
        let input = "0 1";
        let err = Vertex::from_str(input, 2).expect_err("parsing error");
        assert!(
            err.to_string().contains("the row has too few items"),
            "{err}"
        )
    }

    #[test]
    fn try_from_string_too_many_items() {
        let input = "0 1 1 1 1 1 1";
        let err = Vertex::from_str(input, 2).expect_err("parsing error");
        assert!(
            err.to_string().contains("the row has too many items"),
            "{err}"
        )
    }

    #[test]
    fn try_from_string_str_as_id_right() {
        let input = "0 abc 0";
        let err = Vertex::from_str(input, 2).expect_err("parsing error");
        assert!(
            err.to_string().contains("unable to parse the right ID"),
            "{err}"
        )
    }

    #[test]
    fn try_from_string_str_as_id_left() {
        let input = "abc 0 0";
        let err = Vertex::from_str(input, 2).expect_err("parsing error");
        assert!(
            err.to_string().contains("unable to parse the left ID"),
            "{err}"
        )
    }

    #[test]
    fn try_from_string_empty_line() {
        let input = "";
        let err = Vertex::from_str(input, 2).expect_err("parsing error");
        assert!(
            err.to_string().contains("the row has too few items"),
            "{err}"
        )
    }
}
