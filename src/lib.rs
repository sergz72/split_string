use std::collections::HashSet;
use std::io::{Error, ErrorKind};

#[derive(PartialEq, Debug)]
pub struct SplitResult {
    pub result: String,
    pub escaped_with: Option<char>
}

pub fn split_string(source: String, escapes: HashSet<char>) -> Result<Vec<SplitResult>, Error> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut escape: Option<char> = None;
    for c in source.chars() {
        if let Some(e) = escape {
            if e == c {
                result.push(SplitResult{ result: current.clone(), escaped_with: escape });
                current.clear();
                escape = None;
            } else {
                current.push(c);
            }
        } else {
            if c == ' ' {
                if current.len() > 0 {
                    result.push(SplitResult{ result: current.clone(), escaped_with: None });
                    current.clear();
                }
                continue;
            }
            if escapes.contains(&c) {
                escape = Some(c);
                continue;
            }
            current.push(c);
        }
    }
    if escape.is_some() {
        return Err(Error::new(ErrorKind::InvalidInput, "unexpected end of string"));
    }
    if current.len() > 0 {
        result.push(SplitResult{ result: current, escaped_with: None });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::io::Error;
    use crate::{split_string, SplitResult};

    #[test]
    fn test_split_string1() -> Result<(), Error> {
        let result1 = split_string("a b c".to_string(), HashSet::from(['"']))?;
        assert_eq!(result1.len(), 3);
        assert_eq!(result1[0], SplitResult{ result: "a".to_string(), escaped_with: None });
        assert_eq!(result1[1], SplitResult{ result: "b".to_string(), escaped_with: None });
        assert_eq!(result1[2], SplitResult{ result: "c".to_string(), escaped_with: None });
        Ok(())
    }

    #[test]
    fn test_split_string2() -> Result<(), Error> {
        let result1 = split_string("'a b' e 'c d'".to_string(), HashSet::from(['\'']))?;
        assert_eq!(result1.len(), 3);
        assert_eq!(result1[0], SplitResult{ result: "a b".to_string(), escaped_with: Some('\'') });
        assert_eq!(result1[1], SplitResult{ result: "e".to_string(), escaped_with: None });
        assert_eq!(result1[2], SplitResult{ result: "c d".to_string(), escaped_with: Some('\'') });
        Ok(())
    }

    #[test]
    fn test_split_string3() {
        let result1 = split_string("a 'b c".to_string(), HashSet::from(['\'']));
        assert!(result1.is_err());
    }

    #[test]
    fn test_split_string4() -> Result<(), Error> {
        let result1 = split_string("'a b' e \"c d\"".to_string(), HashSet::from(['\'','"']))?;
        assert_eq!(result1.len(), 3);
        assert_eq!(result1[0], SplitResult{ result: "a b".to_string(), escaped_with: Some('\'') });
        assert_eq!(result1[1], SplitResult{ result: "e".to_string(), escaped_with: None });
        assert_eq!(result1[2], SplitResult{ result: "c d".to_string(), escaped_with: Some('"') });
        Ok(())
    }
}
