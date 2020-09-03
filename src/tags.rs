

/// Universal Part-of-speech tags following the [Universal Dependencies
/// scheme](https://universaldependencies.org/u/pos). 
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Universal {
    /// Examples: big, old, green, incomprehensible, first
    Adjective,
    /// Examples: in, to, during
    Adposition,
    /// Examples: very, tomorrow, down, where, there
    Adverb,
    /// Examples: is, has (done), will (do), should (do)
    Auxiliary,
    /// Examples: and, or, but
    Conjunction,
    ///and, or, but
    CoordinatingConjunction,
    /// Examples: a, an, the
    Determiner,
    /// Examples: psst, ouch, bravo, hello
    Interjection,
    /// Examples: girl, cat, tree, air, beauty
    Noun,
    /// Examples: 1, 2020, one, seventy-seven, IV, MMXIV
    Numeral,
    /// Examples: 's, not
    Particle,
    /// Examples: I, you, he, she, myself, themselves, somebody
    Pronoun,
    /// Examples: Mary, John, London, NATO, HBO, Ferris
    ProperNoun,
    /// Examples: ., (, ), ?
    Punctuation,
    /// Examples: if, while, that
    SubordinatingConjunction,
    /// Examples: %, $, £, -, +, =
    Symbol,
    /// Examples: Run, runs, running, eat, ate, eating
    Verb,
    /// Examples: sdijfiodufildug
    Other,
    /// Examples: ' '
    Space
}
