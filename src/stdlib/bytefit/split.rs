/// Splitting functions for byte slices
use super::{ByteFitResult, invalid_utf8};

/// Split slices s into all subslices separated by sep and returns a slice of those subslices.
pub fn split(s: &[u8], sep: &[u8]) -> Vec<Vec<u8>> {
    split_n(s, sep, usize::MAX)
}

/// SplitN slices s into subslices separated by sep and returns a slice of those subslices.
/// The count determines the number of subslices to return:
/// - n > 0: at most n subslices; the last subslice will be the unsplit remainder
/// - n == 0: the result is nil (zero subslices)
/// - n < 0: all subslices
pub fn split_n(s: &[u8], sep: &[u8], n: usize) -> Vec<Vec<u8>> {
    if n == 0 {
        return vec![];
    }
    
    if sep.is_empty() {
        // Split every character
        if n == 1 {
            return vec![s.to_vec()];
        }
        
        let mut result = Vec::new();
        for (i, &byte) in s.iter().enumerate() {
            if result.len() >= n - 1 {
                result.push(s[i..].to_vec());
                break;
            }
            result.push(vec![byte]);
        }
        
        if result.is_empty() && !s.is_empty() {
            result.push(s.to_vec());
        }
        
        return result;
    }
    
    let mut result = Vec::new();
    let mut start = 0;
    
    while start <= s.len() && result.len() < n - 1 {
        if let Some(pos) = find_pattern(&s[start..], sep) {
            let actual_pos = start + pos;
            result.push(s[start..actual_pos].to_vec());
            start = actual_pos + sep.len();
        } else {
            break;
        }
    }
    
    // Add the remaining part
    result.push(s[start..].to_vec());
    
    result
}

/// SplitAfter slices s into subslices after each instance of sep and returns a slice of those subslices.
pub fn split_after(s: &[u8], sep: &[u8]) -> Vec<Vec<u8>> {
    split_after_n(s, sep, usize::MAX)
}

/// SplitAfterN slices s into subslices after each instance of sep and returns a slice of those subslices.
pub fn split_after_n(s: &[u8], sep: &[u8], n: usize) -> Vec<Vec<u8>> {
    if n == 0 {
        return vec![];
    }
    
    if sep.is_empty() {
        // Split after every character
        if n == 1 {
            return vec![s.to_vec()];
        }
        
        let mut result = Vec::new();
        for (i, &byte) in s.iter().enumerate() {
            if result.len() >= n - 1 {
                result.push(s[i..].to_vec());
                break;
            }
            result.push(vec![byte]);
        }
        
        if result.is_empty() && !s.is_empty() {
            result.push(s.to_vec());
        }
        
        return result;
    }
    
    let mut result = Vec::new();
    let mut start = 0;
    
    while start < s.len() && result.len() < n - 1 {
        if let Some(pos) = find_pattern(&s[start..], sep) {
            let actual_pos = start + pos;
            result.push(s[start..actual_pos + sep.len()].to_vec());
            start = actual_pos + sep.len();
        } else {
            break;
        }
    }
    
    // Add the remaining part if any
    if start < s.len() {
        result.push(s[start..].to_vec());
    }
    
    result
}

/// Fields splits the byte slice s around each instance of one or more consecutive white space characters.
pub fn fields(s: &[u8]) -> ByteFitResult<Vec<Vec<u8>>> {
    fields_func(s, |c| c.is_whitespace())
}

/// FieldsFunc splits the byte slice s at each run of Unicode code points c satisfying f(c).
pub fn fields_func<F>(s: &[u8], f: F) -> ByteFitResult<Vec<Vec<u8>>>
where
    F: Fn(char) -> bool,
{
    match std::str::from_utf8(s) {
        Ok(string) => {
            let mut result = Vec::new();
            let mut current = Vec::new();
            let mut in_field = false;
            
            for c in string.chars() {
                if f(c) {
                    if in_field {
                        result.push(current);
                        current = Vec::new();
                        in_field = false;
                    }
                } else {
                    current.extend(c.to_string().as_bytes());
                    in_field = true;
                }
            }
            
            if in_field {
                result.push(current);
            }
            
            Ok(result)
        }
        Err(e) => Err(invalid_utf8(&format!("Invalid UTF-8 sequence: {}", e))),
    }
}

/// Helper function to find a pattern in a byte slice
fn find_pattern(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || needle.len() > haystack.len() {
        return None;
    }
    
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let result = split(b"a,b,c", b",");
        assert_eq!(result, vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]);
        
        let result = split(b"hello", b",");
        assert_eq!(result, vec![b"hello".to_vec()]);
        
        let result = split(b"a,,b", b",");
        assert_eq!(result, vec![b"a".to_vec(), b"".to_vec(), b"b".to_vec()]);
        
        let result = split(b"", b",");
        assert_eq!(result, vec![b"".to_vec()]);
    }

    #[test]
    fn test_split_n() {
        let result = split_n(b"a,b,c,d", b",", 2);
        assert_eq!(result, vec![b"a".to_vec(), b"b,c,d".to_vec()]);
        
        let result = split_n(b"a,b,c", b",", 0);
        assert_eq!(result, Vec::<Vec<u8>>::new());
        
        let result = split_n(b"hello", b"", 3);
        assert_eq!(result, vec![b"h".to_vec(), b"e".to_vec(), b"llo".to_vec()]);
    }

    #[test]
    fn test_split_after() {
        let result = split_after(b"a,b,c", b",");
        assert_eq!(result, vec![b"a,".to_vec(), b"b,".to_vec(), b"c".to_vec()]);
        
        let result = split_after(b"hello", b",");
        assert_eq!(result, vec![b"hello".to_vec()]);
    }

    #[test]
    fn test_split_after_n() {
        let result = split_after_n(b"a,b,c,d", b",", 2);
        assert_eq!(result, vec![b"a,".to_vec(), b"b,c,d".to_vec()]);
        
        let result = split_after_n(b"a,b,c", b",", 0);
        assert_eq!(result, Vec::<Vec<u8>>::new());
    }

    #[test]
    fn test_fields() {
        let result = fields(b"  hello   world  ").unwrap();
        assert_eq!(result, vec![b"hello".to_vec(), b"world".to_vec()]);
        
        let result = fields(b"hello").unwrap();
        assert_eq!(result, vec![b"hello".to_vec()]);
        
        let result = fields(b"   ").unwrap();
        assert_eq!(result, Vec::<Vec<u8>>::new());
        
        let result = fields(b"").unwrap();
        assert_eq!(result, Vec::<Vec<u8>>::new());
    }

    #[test]
    fn test_fields_func() {
        let result = fields_func(b"a:b:c", |c| c == ':').unwrap();
        assert_eq!(result, vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]);
        
        let result = fields_func(b"123abc456", |c| c.is_numeric()).unwrap();
        assert_eq!(result, vec![b"abc".to_vec()]);
    }
}
