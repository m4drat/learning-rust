// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::ops::Rem;

type MatcherFn<T> = fn(T) -> bool;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: MatcherFn<T>,
    substitute: String,
}

impl<T> Matcher<T> {
    pub fn new(_matcher: MatcherFn<T>, _subs: &str) -> Matcher<T> {
        Self {
            matcher: _matcher,
            substitute: _subs.to_string(),
        }
    }

    pub fn check_match(&self, elem: T) -> bool {
        (self.matcher)(elem)
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
#[derive(Default)]
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: ToString + Copy,
{
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |elem| {
            let mut string = "".to_string();
            for matcher in &self.matchers {
                if matcher.check_match(elem) {
                    string += &matcher.substitute;
                }
            }

            if string.is_empty() {
                string = elem.to_string();
            }

            string
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: From<u8> + PartialEq + Rem<Output = T> + Copy + Default + ToString,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % 3.into() == T::default(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == T::default(), "buzz"))
}
