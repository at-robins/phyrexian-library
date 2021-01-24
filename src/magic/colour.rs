//! The 'colour' module provides structures for card colour classification.

use std::borrow::Borrow;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;

// The literal representation of all the supported colours.
const COLOUR_BLACK: &str = "B";
const COLOUR_BLUE: &str = "U";
const COLOUR_GREEN: &str = "G";
const COLOUR_RED: &str = "R";
const COLOUR_WHITE: &str = "W";
// The literal representation of all the supported types of mana.
const MANA_COLOURLESS : &str = "C";
const MANA_MONO_HYBRID: &str = "2/";
const MANA_DUAL_HYBRID: &str = "/";
const MANA_SNOW: &str = "S";
const MANA_PHYREXIAN: &str = "/P";
const MANA_HALF: &str = "H";
// The literal representation of starting and ending delimeters of mana notation.
const MANA_SPECIFIER_START: &str = "{";
const MANA_SPECIFIER_END: &str = "}";
// The literal representation of generic mana notation.
const GENERIC_MANA_INFINITY: &str = "∞";
const GENERIC_MANA_HALF: &str = "½";
const GENERIC_MANA_VARIABLE: [&str; 3] = ["X", "Y", "Z"];


/// The 'Colour' of a Magic product.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Colour {
    Black,
    Blue,
    Green,
    Red,
    White,
}

impl From<Colour> for &str {
    fn from(colour: Colour) -> Self {
        (&colour).into()
    }
}

impl From<&Colour> for &str {
    fn from(colour: &Colour) -> Self {
        match colour {
            Colour::Black => COLOUR_BLACK,
            Colour::Blue => COLOUR_BLUE,
            Colour::Green => COLOUR_GREEN,
            Colour::Red => COLOUR_RED,
            Colour::White => COLOUR_WHITE,
        }
    }
}

impl From<Colour> for String {
    fn from(colour: Colour) -> Self {
        let intermediate: &str = colour.into();
        intermediate.to_string()
    }
}

impl From<&Colour> for String {
    fn from(colour: &Colour) -> Self {
        let intermediate: &str = colour.into();
        intermediate.to_string()
    }
}

impl TryFrom<&str> for Colour {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            COLOUR_BLACK => Ok(Colour::Black),
            COLOUR_BLUE => Ok(Colour::Blue),
            COLOUR_GREEN => Ok(Colour::Green),
            COLOUR_RED => Ok(Colour::Red),
            COLOUR_WHITE => Ok(Colour::White),
            _ => Err(format!("{} is not a valid colour.", value)),
        }
    }
}

impl TryFrom<String> for Colour {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Colour::try_from(value.as_str())
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

/// A set of ['Colour'](phyrexian_library::magic::colour::Colour)s.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColourSet {
    colours: HashSet<Colour>,
}

impl ColourSet {
    /// Create a new set of ['Colour'](phyrexian_library::magic::colour::Colour)s.
    pub fn new() -> Self {
        Self {colours: HashSet::new()}
    }

    /// Checks if the colour set contains the specified
    /// ['Colour'](phyrexian_library::magic::colour::Colour).
    ///
    /// # Parameters
    ///
    /// * `colour` - the ['Colour'](phyrexian_library::magic::colour::Colour) to check
    ///
    /// # Examples
    ///
    /// ```
    /// use phyrexian_library::magic::colour::{Colour, ColourSet};
    ///
    /// let mut colours = ColourSet::new();
    /// assert!(!colours.has(Colour::Blue));
    /// assert!(colours.add(Colour::Blue));
    /// assert!(colours.has(Colour::Blue));
    /// ```
    pub fn has<T: Borrow<Colour>>(&self, colour: T) -> bool {
        self.colours.contains(colour.borrow())
    }

    /// Returns the number of ['Colour'](phyrexian_library::magic::colour::Colour)s in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use phyrexian_library::magic::colour::{Colour, ColourSet};
    ///
    /// let mut colours = ColourSet::new();
    /// assert_eq!(0, colours.length());
    /// colours.add(Colour::Black);
    /// assert_eq!(1, colours.length());
    /// colours.add(Colour::Blue);
    /// assert_eq!(2, colours.length());
    /// colours.add(Colour::Green);
    /// assert_eq!(3, colours.length());
    /// colours.add(Colour::Red);
    /// assert_eq!(4, colours.length());
    /// colours.add(Colour::White);
    /// assert_eq!(5, colours.length());
    /// ```
    pub fn length(&self) -> usize {
        self.colours.len()
    }

    /// Checks if the set has multiple ['Colour'](phyrexian_library::magic::colour::Colour)s.
    ///
    /// # Examples
    ///
    /// ```
    /// use phyrexian_library::magic::colour::{Colour, ColourSet};
    ///
    /// let mut colours = ColourSet::new();
    /// assert!(!colours.is_multicoloured());
    /// colours.add(Colour::Black);
    /// assert!(!colours.is_multicoloured());
    /// colours.add(Colour::Blue);
    /// assert!(colours.is_multicoloured());
    /// ```
    pub fn is_multicoloured(&self) -> bool {
        self.length() > 1
    }

    /// Checks if the set has no ['Colour'](phyrexian_library::magic::colour::Colour)s.
    ///
    /// # Examples
    ///
    /// ```
    /// use phyrexian_library::magic::colour::{Colour, ColourSet};
    ///
    /// let mut colours = ColourSet::new();
    /// assert!(colours.is_colourless());
    /// colours.add(Colour::Black);
    /// assert!(!colours.is_colourless());
    /// ```
    pub fn is_colourless(&self) -> bool {
        self.length() == 0
    }

    /// Checks if the set has only a single ['Colour'](phyrexian_library::magic::colour::Colour).
    ///
    /// # Examples
    ///
    /// ```
    /// use phyrexian_library::magic::colour::{Colour, ColourSet};
    ///
    /// let mut colours = ColourSet::new();
    /// assert!(!colours.is_monocoloured());
    /// colours.add(Colour::Black);
    /// assert!(colours.is_monocoloured());
    /// colours.add(Colour::Blue);
    /// assert!(!colours.is_monocoloured());
    /// ```
    pub fn is_monocoloured(&self) -> bool {
        self.length() == 1
    }

    /// Checks if the specified `ColourSet` is a subset of this set.
    ///
    /// # Parameters
    ///
    /// * `colours` - the subset to validate
    ///
    /// # Examples
    ///
    /// ```
    /// use phyrexian_library::magic::colour::{Colour, ColourSet};
    ///
    /// let mut superset = ColourSet::new();
    /// superset.add(Colour::Black);
    /// superset.add(Colour::Blue);
    /// subset.add(Colour::Black);
    /// assert!(!colours.is_monocoloured());
    /// assert!(colours.is_monocoloured());
    /// colours.add(Colour::Blue);
    /// assert!(!colours.is_monocoloured());
    /// ```
    pub fn is_subset<T: Borrow<ColourSet>>(&self, colours: T) -> bool {
        let mut result = true;
        for c in colours.borrow() {
            result = result && self.has(c);
        }
        result
    }

    /// Adds the specified ['Colour'](phyrexian_library::magic::colour::Colour) to the set.
    /// Returns true if the ['Colour'](phyrexian_library::magic::colour::Colour) was not already
    /// contained in the set, false otherwise.
    ///
    /// # Parameters
    ///
    /// * colour - the ['Colour'](phyrexian_library::magic::colour::Colour) to add
    ///
    /// # Examples
    ///
    /// ```
    /// use phyrexian_library::magic::colour::{Colour, ColourSet};
    ///
    /// let mut colours = ColourSet::new();
    /// assert!(!colours.has(Colour::Red));
    /// assert!(colours.add(Colour::Red));
    /// assert!(colours.has(Colour::Red));
    /// assert!(!colours.add(Colour::Red));
    /// assert!(colours.has(Colour::Red));
    /// ```
    pub fn add(&mut self, colour: Colour) -> bool {
        self.colours.insert(colour)
    }
}

impl<'a> IntoIterator for &'a ColourSet {
    type Item = &'a Colour;
    type IntoIter = std::collections::hash_set::Iter<'a, Colour>;

    fn into_iter(self) -> Self::IntoIter {
        self.colours.iter()
    }
}

impl IntoIterator for ColourSet {
    type Item = Colour;
    type IntoIter = std::collections::hash_set::IntoIter<Colour>;

    fn into_iter(self) -> Self::IntoIter {
        self.colours.into_iter()
    }
}

impl Default for ColourSet {
    fn default() -> Self {
        ColourSet::new()
    }
}

impl FromIterator<Colour> for ColourSet {
    fn from_iter<I: IntoIterator<Item = Colour>>(iter: I) -> ColourSet {
        let mut c = ColourSet::new();
        c.colours.extend(iter);
        c
    }
}

impl fmt::Display for ColourSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = if self.is_colourless() {
            "Colorless"
        } else if self.is_monocoloured() {
            if self.has(Colour::Black) {"Black"}
            else if self.has(Colour::Blue) {"Blue"}
            else if self.has(Colour::Green) {"Green"}
            else if self.has(Colour::Red) {"Red"}
            else {"White"}
        } else if self.length() == 2 {
            if self.has(Colour::Black) && self.has(Colour::Blue) {"House Dimir"}
            else if self.has(Colour::Black) && self.has(Colour::Green) {"Golgari Swarm"}
            else if self.has(Colour::Black) && self.has(Colour::Red) {"Cult of Rakdos"}
            else if self.has(Colour::Black) && self.has(Colour::White) {"Orzhov Syndicate"}
            else if self.has(Colour::Blue) && self.has(Colour::Green) {"Simic Combine"}
            else if self.has(Colour::Blue) && self.has(Colour::Red) {"Izzet League"}
            else if self.has(Colour::Blue) && self.has(Colour::White) {"Azorius Senate"}
            else if self.has(Colour::Green) && self.has(Colour::Red) {"Gruul Clans"}
            else if self.has(Colour::Green) && self.has(Colour::White) {"Selesnya Conclave"}
            else {"Boros Legion"}
        } else if self.length() == 3 {
            if !self.has(Colour::Black) && !self.has(Colour::Blue) {"Naya"}
            else if !self.has(Colour::Black) && !self.has(Colour::Green) {"Jeskai"}
            else if !self.has(Colour::Black) && !self.has(Colour::Red) {"Bant"}
            else if !self.has(Colour::Black) && !self.has(Colour::White) {"Temur"}
            else if !self.has(Colour::Blue) && !self.has(Colour::Green) {"Mardu"}
            else if !self.has(Colour::Blue) && !self.has(Colour::Red) {"Abzan"}
            else if !self.has(Colour::Blue) && !self.has(Colour::White) {"Jund"}
            else if !self.has(Colour::Green) && !self.has(Colour::Red) {"Esper"}
            else if !self.has(Colour::Green) && !self.has(Colour::White) {"Grixis"}
            else {"Sultai"}
        } else if self.length() == 4 {
            if !self.has(Colour::Black) {"Altruism"}
            else if !self.has(Colour::Blue) {"Aggression"}
            else if !self.has(Colour::Green) {"Artifice"}
            else if !self.has(Colour::Red) {"Growth"}
            else {"Chaos"}
        } else {
            "WUBRG"
        };
        f.write_str(s)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// A type of `Mana`.
pub enum Mana {
    Coloured(Colour),
    Colourless,
    Generic(GenericCost),
    MonoHybrid(Colour),
    DualHybrid(Colour, Colour),
    Snow,
    Phyrexian(Colour),
    Half(Colour),
}

impl Mana {
    /// Returns the converted mana cost.
    pub fn converted_mana_cost(&self) -> f64 {
        match self {
            Mana::Coloured(_) => 1.0,
            Mana::Colourless => 1.0,
            Mana::Generic(gen) => gen.converted_mana_cost(),
            Mana::MonoHybrid(_) => 2.0,
            Mana::DualHybrid(_,_) => 1.0,
            Mana::Snow => 1.0,
            Mana::Phyrexian(_) => 1.0,
            Mana::Half(_) => 0.5,
        }
    }

    /// Tries to convert a string without specifiers into coloured mana.
    ///
    /// # Parameters
    ///
    /// * `value` - the string to convert
    fn into_coloured(value: &str) -> Option<Mana> {
        Colour::try_from(value).ok().map(|col| Mana::Coloured(col))
    }

    /// Tries to convert a string without specifiers into colourless mana.
    ///
    /// # Parameters
    ///
    /// * `value` - the string to convert
    fn into_colourless(value: &str) -> Option<Mana> {
        match value {
            MANA_COLOURLESS => Some(Mana::Colourless),
            _ => None,
        }
    }

    /// Tries to convert a string without specifiers into generic mana.
    ///
    /// # Parameters
    ///
    /// * `value` - the string to convert
    fn into_generic(value: &str) -> Option<Mana> {
        GenericCost::try_from(value).ok().map(|gen| Mana::Generic(gen))
    }

    /// Tries to convert a string without specifiers into mono hybrid mana.
    ///
    /// # Parameters
    ///
    /// * `value` - the string to convert
    fn into_mono_hybrid(value: &str) -> Option<Mana> {
        value.strip_prefix(MANA_MONO_HYBRID)
            .and_then(|stripped| Colour::try_from(stripped).ok().map(|col| Mana::MonoHybrid(col)))
    }

    /// Tries to convert a string without specifiers into dual hybrid mana.
    ///
    /// # Parameters
    ///
    /// * `value` - the string to convert
    fn into_dual_hybrid(value: &str) -> Option<Mana> {
        let mut colours: Vec<Result<Colour, String>> = value.splitn(2, MANA_DUAL_HYBRID)
            .map(|split| Colour::try_from(split))
            .collect();
        if colours.len() == 2 {
            match (colours.remove(0), colours.remove(0)) {
                (Ok(a), Ok(b)) => Some(Mana::DualHybrid(a, b)),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Tries to convert a string without specifiers into phyrexian mana.
    ///
    /// # Parameters
    ///
    /// * `value` - the string to convert
    fn into_phyrexian(value: &str) -> Option<Mana> {
        value.strip_suffix(MANA_PHYREXIAN)
            .and_then(|stripped| Colour::try_from(stripped).ok().map(|col| Mana::Phyrexian(col)))
    }

    /// Tries to convert a string without specifiers into half mana.
    ///
    /// # Parameters
    ///
    /// * `value` - the string to convert
    fn into_half(value: &str) -> Option<Mana> {
        value.strip_prefix(MANA_HALF)
            .and_then(|stripped| Colour::try_from(stripped).ok().map(|col| Mana::Half(col)))
    }

    /// Tries to convert a string without specifiers into snow mana.
    ///
    /// # Parameters
    ///
    /// * `value` - the string to convert
    fn into_snow(value: &str) -> Option<Mana> {
        match value {
            MANA_SNOW => Some(Mana::Colourless),
            _ => None,
        }
    }
}

impl From<Mana> for String {
    fn from(mana: Mana) -> Self {
        format!("{}", mana)
    }
}

impl From<&Mana> for String {
    fn from(mana: &Mana) -> Self {
        format!("{}", mana)
    }
}

impl TryFrom<&str> for Mana {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.strip_prefix(MANA_SPECIFIER_START)
            .and_then(|trim| trim.strip_suffix(MANA_SPECIFIER_END))
            .and_then(|inner| -> Option<Mana> {
                Mana::into_coloured(inner)
                    .or(Mana::into_colourless(inner))
                    .or(Mana::into_generic(inner))
                    .or(Mana::into_mono_hybrid(inner))
                    .or(Mana::into_dual_hybrid(inner))
                    .or(Mana::into_snow(inner))
                    .or(Mana::into_phyrexian(inner))
                    .or(Mana::into_half(inner))
            })
            .ok_or(format!("{} is not valid mana.", value))
    }
}

impl TryFrom<String> for Mana {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

impl fmt::Display for Mana {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner: String = match self {
            Mana::Coloured(colour) => colour.into(),
            Mana::Colourless => MANA_COLOURLESS.to_string(),
            Mana::Generic(amount) => amount.into(),
            Mana::MonoHybrid(colour) => format!("{}{}", MANA_MONO_HYBRID, colour),
            Mana::DualHybrid(colour_a,colour_b) => format!("{}{}{}", colour_a, MANA_DUAL_HYBRID, colour_b),
            Mana::Snow => MANA_SNOW.to_string(),
            Mana::Phyrexian(colour) => format!("{}{}", colour, MANA_PHYREXIAN),
            Mana::Half(colour) => format!("{}{}", MANA_HALF, colour),
        };
        write!(f, "{}{}{}", MANA_SPECIFIER_START, inner, MANA_SPECIFIER_END)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// The mana cost of a card.
pub struct ManaCost {
    mana: Vec<Mana>,
}

impl ManaCost {
    /// Creates a new 'ManaCost'.
    ///
    /// # Parameters
    ///
    /// * mana - the mana cost
    pub fn new(mana: Vec<Mana>) -> Self {
        Self {mana}
    }

    /// Returns the converted mana cost.
    pub fn converted_mana_cost(&self) -> f64 {
        self.mana.iter().map(|m| m.converted_mana_cost()).sum()
    }
}

impl fmt::Display for ManaCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for cost in &self.mana {
            let intermediate: String = cost.into();
            s.push_str(&intermediate);
        }
        write!(f, "{}", s)
    }
}

/// The `GenericCost` enum defines types of generic mana costs.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GenericCost {
    Infinity,
    Half,
    Integer(i32),
    Variable(String),
}

impl GenericCost {
    /// Returns the converted mana cost.
    pub fn converted_mana_cost(&self) -> f64 {
        match self {
            GenericCost::Infinity => f64::INFINITY,
            GenericCost::Half => 0.5,
            GenericCost::Integer(amount) => *amount as f64,
            GenericCost::Variable(_) => 0.0,
        }
    }

    /// Tries to convert the specified string to a a `GenericCost::Integer`.
    ///
    /// # Parameters
    ///
    /// * `i` - the potential integer
    fn to_integer(i: &str) -> Result<GenericCost, std::num::ParseIntError> {
        i.parse::<i32>().map(|i| GenericCost::Integer(i))
    }

    /// Tries to convert the specified string to a a `GenericCost::Variable`.
    ///
    /// # Parameters
    ///
    /// * `v` - the potential variable
    fn to_variable(v: &str) -> Result<GenericCost, ()> {
        if GENERIC_MANA_VARIABLE.contains(&v) {
            Ok(GenericCost::Variable(v.to_string()))
        } else {
            Err(())
        }
    }
}

impl From<GenericCost> for String {
    fn from(generic_cost: GenericCost) -> Self {
        String::from(&generic_cost)
    }
}

impl From<&GenericCost> for String {
    fn from(generic_cost: &GenericCost) -> Self {
        match generic_cost {
            GenericCost::Infinity => GENERIC_MANA_INFINITY.to_string(),
            GenericCost::Half => GENERIC_MANA_HALF.to_string(),
            GenericCost::Integer(amount) => amount.to_string(),
            GenericCost::Variable(variable) => variable.to_string(),
        }
    }
}

impl TryFrom<&str> for GenericCost {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            GENERIC_MANA_INFINITY => Ok(GenericCost::Infinity),
            GENERIC_MANA_HALF => Ok(GenericCost::Half),
            complex => {
                GenericCost::to_integer(complex)
                    .or_else(|_| GenericCost::to_variable(complex))
                    .map_err(|_| format!("{} is not a valid generic cost.", value))
            }
        }
    }
}

impl PartialOrd for GenericCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GenericCost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.converted_mana_cost().partial_cmp(&other.converted_mana_cost())
            .expect(&format!("Generic cost {} and {} must be fully comparable.", self, other))
    }
}

impl TryFrom<String> for GenericCost {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        GenericCost::try_from(value.as_str())
    }
}

impl fmt::Display for GenericCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&String::from(self))
    }
}
