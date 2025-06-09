/// fr fr Advanced matchers for the TestVibes framework
use crate::stdlib::packages::test_vibes::core::VibeTest;
use std::fmt::Debug;
use std::collections::HashMap;

/// fr fr Trait for custom matchers
pub trait Matcher<T> {
    fn matches(&self, actual: &T) -> bool;
    fn description(&self) -> String;
    fn failure_message(&self, actual: &T) -> String;
}

/// fr fr Execute a matcher against a value
pub fn expect<T>(t: &mut VibeTest, actual: T, matcher: impl Matcher<T>) {
    if !matcher.matches(&actual) {
        t.fail_vibe(&matcher.failure_message(&actual));
    }
}

/// fr fr Basic value matchers

/// fr fr Equal matcher
pub struct EqualMatcher<T> {
    expected: T,
}

impl<T> EqualMatcher<T> {
    pub fn new(expected: T) -> Self {
        Self { expected }
    }
}

impl<T: PartialEq + Debug> Matcher<T> for EqualMatcher<T> {
    fn matches(&self, actual: &T) -> bool {
        *actual == self.expected
    }

    fn description(&self) -> String {
        format!("equal to {:?}", self.expected)
    }

    fn failure_message(&self, actual: &T) -> String {
        format!("Expected {:?}, but got {:?}", self.expected, actual)
    }
}

/// fr fr Not equal matcher
pub struct NotEqualMatcher<T> {
    unexpected: T,
}

impl<T> NotEqualMatcher<T> {
    pub fn new(unexpected: T) -> Self {
        Self { unexpected }
    }
}

impl<T: PartialEq + Debug> Matcher<T> for NotEqualMatcher<T> {
    fn matches(&self, actual: &T) -> bool {
        *actual != self.unexpected
    }

    fn description(&self) -> String {
        format!("not equal to {:?}", self.unexpected)
    }

    fn failure_message(&self, actual: &T) -> String {
        format!("Expected not to equal {:?}, but it did", self.unexpected)
    }
}

/// fr fr Numeric matchers

/// fr fr Greater than matcher
pub struct GreaterThanMatcher<T> {
    threshold: T,
}

impl<T> GreaterThanMatcher<T> {
    pub fn new(threshold: T) -> Self {
        Self { threshold }
    }
}

impl<T: PartialOrd + Debug> Matcher<T> for GreaterThanMatcher<T> {
    fn matches(&self, actual: &T) -> bool {
        *actual > self.threshold
    }

    fn description(&self) -> String {
        format!("greater than {:?}", self.threshold)
    }

    fn failure_message(&self, actual: &T) -> String {
        format!("Expected {:?} to be greater than {:?}", actual, self.threshold)
    }
}

/// fr fr Less than matcher
pub struct LessThanMatcher<T> {
    threshold: T,
}

impl<T> LessThanMatcher<T> {
    pub fn new(threshold: T) -> Self {
        Self { threshold }
    }
}

impl<T: PartialOrd + Debug> Matcher<T> for LessThanMatcher<T> {
    fn matches(&self, actual: &T) -> bool {
        *actual < self.threshold
    }

    fn description(&self) -> String {
        format!("less than {:?}", self.threshold)
    }

    fn failure_message(&self, actual: &T) -> String {
        format!("Expected {:?} to be less than {:?}", actual, self.threshold)
    }
}

/// fr fr Range matcher
pub struct InRangeMatcher<T> {
    min: T,
    max: T,
    inclusive: bool,
}

impl<T> InRangeMatcher<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max, inclusive: true }
    }

    pub fn exclusive(min: T, max: T) -> Self {
        Self { min, max, inclusive: false }
    }
}

impl<T: PartialOrd + Debug> Matcher<T> for InRangeMatcher<T> {
    fn matches(&self, actual: &T) -> bool {
        if self.inclusive {
            *actual >= self.min && *actual <= self.max
        } else {
            *actual > self.min && *actual < self.max
        }
    }

    fn description(&self) -> String {
        if self.inclusive {
            format!("in range [{:?}, {:?}]", self.min, self.max)
        } else {
            format!("in range ({:?}, {:?})", self.min, self.max)
        }
    }

    fn failure_message(&self, actual: &T) -> String {
        format!("Expected {:?} to be {}", actual, self.description())
    }
}

/// fr fr String matchers

/// fr fr Contains substring matcher
pub struct ContainsSubstringMatcher {
    substring: String,
}

impl ContainsSubstringMatcher {
    pub fn new(substring: &str) -> Self {
        Self { substring: substring.to_string() }
    }
}

impl Matcher<String> for ContainsSubstringMatcher {
    fn matches(&self, actual: &String) -> bool {
        actual.contains(&self.substring)
    }

    fn description(&self) -> String {
        format!("contains substring '{}'", self.substring)
    }

    fn failure_message(&self, actual: &String) -> String {
        format!("Expected '{}' to contain '{}'", actual, self.substring)
    }
}

impl Matcher<&str> for ContainsSubstringMatcher {
    fn matches(&self, actual: &&str) -> bool {
        actual.contains(&self.substring)
    }

    fn description(&self) -> String {
        format!("contains substring '{}'", self.substring)
    }

    fn failure_message(&self, actual: &&str) -> String {
        format!("Expected '{}' to contain '{}'", actual, self.substring)
    }
}

/// fr fr Starts with matcher
pub struct StartsWithMatcher {
    prefix: String,
}

impl StartsWithMatcher {
    pub fn new(prefix: &str) -> Self {
        Self { prefix: prefix.to_string() }
    }
}

impl Matcher<String> for StartsWithMatcher {
    fn matches(&self, actual: &String) -> bool {
        actual.starts_with(&self.prefix)
    }

    fn description(&self) -> String {
        format!("starts with '{}'", self.prefix)
    }

    fn failure_message(&self, actual: &String) -> String {
        format!("Expected '{}' to start with '{}'", actual, self.prefix)
    }
}

/// fr fr Ends with matcher
pub struct EndsWithMatcher {
    suffix: String,
}

impl EndsWithMatcher {
    pub fn new(suffix: &str) -> Self {
        Self { suffix: suffix.to_string() }
    }
}

impl Matcher<String> for EndsWithMatcher {
    fn matches(&self, actual: &String) -> bool {
        actual.ends_with(&self.suffix)
    }

    fn description(&self) -> String {
        format!("ends with '{}'", self.suffix)
    }

    fn failure_message(&self, actual: &String) -> String {
        format!("Expected '{}' to end with '{}'", actual, self.suffix)
    }
}

/// fr fr Regex matcher
pub struct RegexMatcher {
    pattern: String,
}

impl RegexMatcher {
    pub fn new(pattern: &str) -> Self {
        Self { pattern: pattern.to_string() }
    }

    /// fr fr Simple pattern matching (would use regex crate in real implementation)
    fn simple_match(&self, text: &str) -> bool {
        match self.pattern.as_str() {
            r"\d+" => text.chars().all(|c| c.is_ascii_digit()),
            r"\w+" => text.chars().all(|c| c.is_alphanumeric()),
            r"\s+" => text.chars().all(|c| c.is_whitespace()),
            pattern => text.contains(pattern),
        }
    }
}

impl Matcher<String> for RegexMatcher {
    fn matches(&self, actual: &String) -> bool {
        self.simple_match(actual)
    }

    fn description(&self) -> String {
        format!("matches regex '{}'", self.pattern)
    }

    fn failure_message(&self, actual: &String) -> String {
        format!("Expected '{}' to match regex '{}'", actual, self.pattern)
    }
}

/// fr fr Collection matchers

/// fr fr Has length matcher
pub struct HasLengthMatcher {
    expected_length: usize,
}

impl HasLengthMatcher {
    pub fn new(expected_length: usize) -> Self {
        Self { expected_length }
    }
}

impl<T> Matcher<Vec<T>> for HasLengthMatcher {
    fn matches(&self, actual: &Vec<T>) -> bool {
        actual.len() == self.expected_length
    }

    fn description(&self) -> String {
        format!("has length {}", self.expected_length)
    }

    fn failure_message(&self, actual: &Vec<T>) -> String {
        format!("Expected length {}, but got {}", self.expected_length, actual.len())
    }
}

impl Matcher<String> for HasLengthMatcher {
    fn matches(&self, actual: &String) -> bool {
        actual.len() == self.expected_length
    }

    fn description(&self) -> String {
        format!("has length {}", self.expected_length)
    }

    fn failure_message(&self, actual: &String) -> String {
        format!("Expected length {}, but got {}", self.expected_length, actual.len())
    }
}

/// fr fr Is empty matcher
pub struct IsEmptyMatcher;

impl<T> Matcher<Vec<T>> for IsEmptyMatcher {
    fn matches(&self, actual: &Vec<T>) -> bool {
        actual.is_empty()
    }

    fn description(&self) -> String {
        "is empty".to_string()
    }

    fn failure_message(&self, actual: &Vec<T>) -> String {
        format!("Expected empty collection, but got {} items", actual.len())
    }
}

impl Matcher<String> for IsEmptyMatcher {
    fn matches(&self, actual: &String) -> bool {
        actual.is_empty()
    }

    fn description(&self) -> String {
        "is empty".to_string()
    }

    fn failure_message(&self, actual: &String) -> String {
        format!("Expected empty string, but got '{}'", actual)
    }
}

/// fr fr Contains element matcher
pub struct ContainsElementMatcher<T> {
    expected_element: T,
}

impl<T> ContainsElementMatcher<T> {
    pub fn new(expected_element: T) -> Self {
        Self { expected_element }
    }
}

impl<T: PartialEq + Debug> Matcher<Vec<T>> for ContainsElementMatcher<T> {
    fn matches(&self, actual: &Vec<T>) -> bool {
        actual.contains(&self.expected_element)
    }

    fn description(&self) -> String {
        format!("contains element {:?}", self.expected_element)
    }

    fn failure_message(&self, actual: &Vec<T>) -> String {
        format!("Expected collection to contain {:?}, but it didn't. Collection: {:?}", 
                self.expected_element, actual)
    }
}

/// fr fr All elements match matcher
pub struct AllElementsMatcher<T, M> {
    element_matcher: M,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, M> AllElementsMatcher<T, M> {
    pub fn new(element_matcher: M) -> Self {
        Self { 
            element_matcher, 
            _phantom: std::marker::PhantomData 
        }
    }
}

impl<T, M> Matcher<Vec<T>> for AllElementsMatcher<T, M>
where
    M: Matcher<T>,
    T: Debug,
{
    fn matches(&self, actual: &Vec<T>) -> bool {
        actual.iter().all(|element| self.element_matcher.matches(element))
    }

    fn description(&self) -> String {
        format!("all elements {}", self.element_matcher.description())
    }

    fn failure_message(&self, actual: &Vec<T>) -> String {
        format!("Expected all elements to {}, but some didn't. Collection: {:?}", 
                self.element_matcher.description(), actual)
    }
}

/// fr fr Any element matches matcher
pub struct AnyElementMatcher<T, M> {
    element_matcher: M,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, M> AnyElementMatcher<T, M> {
    pub fn new(element_matcher: M) -> Self {
        Self { 
            element_matcher, 
            _phantom: std::marker::PhantomData 
        }
    }
}

impl<T, M> Matcher<Vec<T>> for AnyElementMatcher<T, M>
where
    M: Matcher<T>,
    T: Debug,
{
    fn matches(&self, actual: &Vec<T>) -> bool {
        actual.iter().any(|element| self.element_matcher.matches(element))
    }

    fn description(&self) -> String {
        format!("any element {}", self.element_matcher.description())
    }

    fn failure_message(&self, actual: &Vec<T>) -> String {
        format!("Expected any element to {}, but none did. Collection: {:?}", 
                self.element_matcher.description(), actual)
    }
}

/// fr fr Logical matchers

/// fr fr Not matcher
pub struct NotMatcher<M> {
    inner_matcher: M,
}

impl<M> NotMatcher<M> {
    pub fn new(inner_matcher: M) -> Self {
        Self { inner_matcher }
    }
}

impl<T, M: Matcher<T>> Matcher<T> for NotMatcher<M> {
    fn matches(&self, actual: &T) -> bool {
        !self.inner_matcher.matches(actual)
    }

    fn description(&self) -> String {
        format!("not {}", self.inner_matcher.description())
    }

    fn failure_message(&self, actual: &T) -> String {
        format!("Expected not to {}, but it did", self.inner_matcher.description())
    }
}

/// fr fr And matcher
pub struct AndMatcher<M1, M2> {
    first: M1,
    second: M2,
}

impl<M1, M2> AndMatcher<M1, M2> {
    pub fn new(first: M1, second: M2) -> Self {
        Self { first, second }
    }
}

impl<T, M1: Matcher<T>, M2: Matcher<T>> Matcher<T> for AndMatcher<M1, M2> {
    fn matches(&self, actual: &T) -> bool {
        self.first.matches(actual) && self.second.matches(actual)
    }

    fn description(&self) -> String {
        format!("{} and {}", self.first.description(), self.second.description())
    }

    fn failure_message(&self, actual: &T) -> String {
        if !self.first.matches(actual) {
            self.first.failure_message(actual)
        } else {
            self.second.failure_message(actual)
        }
    }
}

/// fr fr Or matcher
pub struct OrMatcher<M1, M2> {
    first: M1,
    second: M2,
}

impl<M1, M2> OrMatcher<M1, M2> {
    pub fn new(first: M1, second: M2) -> Self {
        Self { first, second }
    }
}

impl<T, M1: Matcher<T>, M2: Matcher<T>> Matcher<T> for OrMatcher<M1, M2> {
    fn matches(&self, actual: &T) -> bool {
        self.first.matches(actual) || self.second.matches(actual)
    }

    fn description(&self) -> String {
        format!("{} or {}", self.first.description(), self.second.description())
    }

    fn failure_message(&self, actual: &T) -> String {
        format!("Expected {} or {}, but neither matched", 
                self.first.description(), self.second.description())
    }
}

/// fr fr Convenience functions for creating matchers

/// fr fr Create an equal matcher
pub fn equal<T>(expected: T) -> EqualMatcher<T> {
    EqualMatcher::new(expected)
}

/// fr fr Create a not equal matcher
pub fn not_equal<T>(unexpected: T) -> NotEqualMatcher<T> {
    NotEqualMatcher::new(unexpected)
}

/// fr fr Create a greater than matcher
pub fn greater_than<T>(threshold: T) -> GreaterThanMatcher<T> {
    GreaterThanMatcher::new(threshold)
}

/// fr fr Create a less than matcher
pub fn less_than<T>(threshold: T) -> LessThanMatcher<T> {
    LessThanMatcher::new(threshold)
}

/// fr fr Create an in-range matcher
pub fn in_range<T>(min: T, max: T) -> InRangeMatcher<T> {
    InRangeMatcher::new(min, max)
}

/// fr fr Create a contains substring matcher
pub fn contains_substring(substring: &str) -> ContainsSubstringMatcher {
    ContainsSubstringMatcher::new(substring)
}

/// fr fr Create a starts with matcher
pub fn starts_with(prefix: &str) -> StartsWithMatcher {
    StartsWithMatcher::new(prefix)
}

/// fr fr Create an ends with matcher
pub fn ends_with(suffix: &str) -> EndsWithMatcher {
    EndsWithMatcher::new(suffix)
}

/// fr fr Create a regex matcher
pub fn matches_regex(pattern: &str) -> RegexMatcher {
    RegexMatcher::new(pattern)
}

/// fr fr Create a has length matcher
pub fn has_length(expected_length: usize) -> HasLengthMatcher {
    HasLengthMatcher::new(expected_length)
}

/// fr fr Create an is empty matcher
pub fn is_empty() -> IsEmptyMatcher {
    IsEmptyMatcher
}

/// fr fr Create a contains element matcher
pub fn contains_element<T>(expected_element: T) -> ContainsElementMatcher<T> {
    ContainsElementMatcher::new(expected_element)
}

/// fr fr Create a not matcher
pub fn not<M>(inner_matcher: M) -> NotMatcher<M> {
    NotMatcher::new(inner_matcher)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::packages::test_vibes::core::VibeTest;

    #[test]
    fn test_equal_matcher() {
        let mut test = VibeTest::new("equal_test".to_string());
        
        expect(&mut test, 42, equal(42));
        assert!(!test.failed());
        
        expect(&mut test, 42, equal(24));
        assert!(test.failed());
    }

    #[test]
    fn test_greater_than_matcher() {
        let mut test = VibeTest::new("greater_test".to_string());
        
        expect(&mut test, 10, greater_than(5));
        assert!(!test.failed());
        
        expect(&mut test, 3, greater_than(5));
        assert!(test.failed());
    }

    #[test]
    fn test_contains_substring_matcher() {
        let mut test = VibeTest::new("contains_test".to_string());
        
        expect(&mut test, "hello world".to_string(), contains_substring("world"));
        assert!(!test.failed());
        
        expect(&mut test, "hello world".to_string(), contains_substring("foo"));
        assert!(test.failed());
    }

    #[test]
    fn test_has_length_matcher() {
        let mut test = VibeTest::new("length_test".to_string());
        
        expect(&mut test, vec![1, 2, 3], has_length(3));
        assert!(!test.failed());
        
        expect(&mut test, vec![1, 2], has_length(3));
        assert!(test.failed());
    }

    #[test]
    fn test_contains_element_matcher() {
        let mut test = VibeTest::new("contains_element_test".to_string());
        
        expect(&mut test, vec![1, 2, 3], contains_element(2));
        assert!(!test.failed());
        
        expect(&mut test, vec![1, 2, 3], contains_element(4));
        assert!(test.failed());
    }

    #[test]
    fn test_not_matcher() {
        let mut test = VibeTest::new("not_test".to_string());
        
        expect(&mut test, 42, not(equal(24)));
        assert!(!test.failed());
        
        expect(&mut test, 42, not(equal(42)));
        assert!(test.failed());
    }

    #[test]
    fn test_and_matcher() {
        let mut test = VibeTest::new("and_test".to_string());
        
        let matcher = AndMatcher::new(greater_than(5), less_than(10));
        expect(&mut test, 7, matcher);
        assert!(!test.failed());
        
        let matcher2 = AndMatcher::new(greater_than(5), less_than(10));
        expect(&mut test, 12, matcher2);
        assert!(test.failed());
    }

    #[test]
    fn test_or_matcher() {
        let mut test = VibeTest::new("or_test".to_string());
        
        let matcher = OrMatcher::new(equal(5), equal(10));
        expect(&mut test, 5, matcher);
        assert!(!test.failed());
        
        let matcher2 = OrMatcher::new(equal(5), equal(10));
        expect(&mut test, 7, matcher2);
        assert!(test.failed());
    }

    #[test]
    fn test_in_range_matcher() {
        let mut test = VibeTest::new("range_test".to_string());
        
        expect(&mut test, 5, in_range(1, 10));
        assert!(!test.failed());
        
        expect(&mut test, 15, in_range(1, 10));
        assert!(test.failed());
    }

    #[test]
    fn test_regex_matcher() {
        let mut test = VibeTest::new("regex_test".to_string());
        
        expect(&mut test, "12345".to_string(), matches_regex(r"\d+"));
        assert!(!test.failed());
        
        expect(&mut test, "hello".to_string(), matches_regex(r"\d+"));
        assert!(test.failed());
    }

    #[test]
    fn test_is_empty_matcher() {
        let mut test = VibeTest::new("empty_test".to_string());
        
        expect(&mut test, Vec::<i32>::new(), is_empty());
        assert!(!test.failed());
        
        expect(&mut test, vec![1, 2, 3], is_empty());
        assert!(test.failed());
    }
}
