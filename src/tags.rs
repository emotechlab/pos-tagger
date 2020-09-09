//! TODO

/// Universal Part-of-speech tags following the [Universal Dependencies
/// scheme](https://universaldependencies.org/u/pos).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UniversalDependencies {
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
    Space,
}

/// Part-of-speech tag set respresenting the [OntoNotes 5](catalog.ldc.upenn.edu/LDC2013T19) tags.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OntoNotes5 {
    Currency,
    OpeningQuotation,
    ClosingQuotation,
    Comma,
    LeftRoundBracket,
    RightRoundBracket,
    SentenceCloser,
    ColonOrEllipsis,
    Email,
    Affix,
    CoordinatingConjunction,
    CardinalNumber,
    Determiner,
    ExistentialThere,
    ForeignWord,
    AdditionalWord,
    Hyphen,
    /// Conjunction subordinating or preposition
    Conjunction,
    Adjective,
    Comparative,
    Superlative,
    ListItemMarker,
    ModalAuxiliary,
    SuperfluousPunctuation,
    MissingTag,
    NounSingularOrPlural,
    NounProperSingular,
    NounProperPlural,
    NounPlural,
    Predeterminer,
    PossessiveEnding,
    PronounPersonal,
    PronounPossessive,
    Adverb,
    AdverbComparative,
    AdverbSuperlative,
    AdverbParticle,
    Space,
    Symbol,
    InfinitivalTo,
    Interjection,
    VerbBaseForm,
    VerbPastTense,
    VerbGerundOrPresentParticiple,
    VerbNonThirdPersonSingularPresent,
    WhDeterminer,
    WhPronounPersonal,
    WhPronounPossessive,
    WhAdverb,
    Unknown,
}

impl From<OntoNotes5> for UniversalDependencies {
    fn from(onto: OntoNotes5) -> Self {
        use OntoNotes5::*;
        match onto {
            Currency => Self::Symbol,
            OpeningQuotation => Self::Punctuation,
            ClosingQuotation => Self::Punctuation,
            Comma => Self::Punctuation,
            LeftRoundBracket => Self::Punctuation,
            RightRoundBracket => Self::Punctuation,
            SentenceCloser => Self::Punctuation,
            ColonOrEllipsis => Self::Punctuation,
            Email => Self::Other,
            Affix => Self::Adjective,
            CoordinatingConjunction => Self::CoordinatingConjunction,
            CardinalNumber => Self::Numeral,
            Determiner => Self::Determiner,
            ExistentialThere => Self::Pronoun,
            ForeignWord => Self::Other,
            AdditionalWord => Self::Other,
            Hyphen => Self::Punctuation,
            Conjunction => Self::Adposition,
            Adjective => Self::Adjective,
            Comparative => Self::Adjective,
            Superlative => Self::Adjective,
            ListItemMarker => Self::Other,
            ModalAuxiliary => Self::Verb,
            SuperfluousPunctuation => Self::Punctuation,
            MissingTag => Self::Other,
            NounSingularOrPlural => Self::Noun,
            NounProperSingular => Self::ProperNoun,
            NounProperPlural => Self::ProperNoun,
            NounPlural => Self::Noun,
            Predeterminer => Self::Determiner,
            PossessiveEnding => Self::Particle,
            PronounPersonal => Self::Pronoun,
            PronounPossessive => Self::Determiner,
            Adverb => Self::Adverb,
            AdverbComparative => Self::Adverb,
            AdverbSuperlative => Self::Adverb,
            AdverbParticle => Self::Adposition,
            Space => Self::Space,
            Symbol => Self::Symbol,
            InfinitivalTo => Self::Particle,
            Interjection => Self::Interjection,
            VerbBaseForm => Self::Verb,
            VerbPastTense => Self::Verb,
            VerbGerundOrPresentParticiple => Self::Verb,
            VerbNonThirdPersonSingularPresent => Self::Verb,
            WhDeterminer => Self::Determiner,
            WhPronounPersonal => Self::Pronoun,
            WhPronounPossessive => Self::Determiner,
            WhAdverb => Self::Adverb,
            Unknown => Self::Other,
        }
    }
}
