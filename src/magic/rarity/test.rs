use super::*;
use std::convert::TryInto;

#[test]
/// Tests if the ordering of `Rarity` works as expected.
fn test_ordering() {
    let mut unordered = vec!(
        Rarity::Common,
        Rarity::Mythic,
        Rarity::Common,
        Rarity::Rare,
        Rarity::Uncommon,
        Rarity::Mythic,
    );
    let ordered = vec!(
        Rarity::Common,
        Rarity::Common,
        Rarity::Uncommon,
        Rarity::Rare,
        Rarity::Mythic,
        Rarity::Mythic,
    );
    assert_ne!(unordered, ordered);
    unordered.sort();
    assert_eq!(unordered, ordered);
}

#[test]
/// Tests if the conversion from `Rarity` to string works as expected.
fn test_conversion_to_string() {
    assert_eq!(Into::<&str>::into(Rarity::Common), RARITY_COMMON);
    assert_eq!(Into::<&str>::into(Rarity::Uncommon), RARITY_UNCOMMON);
    assert_eq!(Into::<&str>::into(Rarity::Rare), RARITY_RARE);
    assert_eq!(Into::<&str>::into(Rarity::Mythic), RARITY_MYTHIC);
}

#[test]
/// Tests if the conversion from string to `Rarity` works as expected.
fn test_conversion_from_string() {
    assert_eq!(TryInto::<Rarity>::try_into(RARITY_COMMON), Ok(Rarity::Common));
    assert_eq!(TryInto::<Rarity>::try_into(RARITY_UNCOMMON), Ok(Rarity::Uncommon));
    assert_eq!(TryInto::<Rarity>::try_into(RARITY_RARE), Ok(Rarity::Rare));
    assert_eq!(TryInto::<Rarity>::try_into(RARITY_MYTHIC), Ok(Rarity::Mythic));
    assert_eq!(TryInto::<Rarity>::try_into(RARITY_COMMON.to_string()), Ok(Rarity::Common));
    assert_eq!(TryInto::<Rarity>::try_into(RARITY_UNCOMMON.to_string()), Ok(Rarity::Uncommon));
    assert_eq!(TryInto::<Rarity>::try_into(RARITY_RARE.to_string()), Ok(Rarity::Rare));
    assert_eq!(TryInto::<Rarity>::try_into(RARITY_MYTHIC.to_string()), Ok(Rarity::Mythic));
}
