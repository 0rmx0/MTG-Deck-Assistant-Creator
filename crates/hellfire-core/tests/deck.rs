use hellfire_core::card::{Card, Colors};
use hellfire_core::{Deck, Format, ManaCost};

fn stub(id: &str, name: &str, types: &str, cost: Option<u32>) -> Card {
    Card {
        scryfall_id: id.into(),
        name: name.into(),
        mana_cost: cost.map(|c| ManaCost {
            generic: c,
            ..Default::default()
        }),
        type_line: types.into(),
        oracle_text: String::new(),
        colors: Colors::default(),
        color_identity: Colors::default(),
        set_code: "TST".into(),
        rarity: "common".into(),
        price_usd_cents: 100,
        legalities: Default::default(),
    }
}

#[test]
fn recruit_and_sabotage() {
    let mut d = Deck::new("Insurrection", Format::Modern);
    let bolt = stub("1", "Lightning Bolt", "Instant", Some(1));
    d.recruit(bolt.clone(), 4, false).unwrap();
    assert_eq!(d.stats().total_cards, 4);
    d.sabotage("1", 2).unwrap();
    assert_eq!(d.stats().total_cards, 2);
    assert!(d.recruit(bolt, 3, false).is_err());
}

#[test]
fn validate_min_size() {
    let mut d = Deck::new("Manifestation", Format::Modern);
    d.recruit(stub("2", "Rag Man", "Creature — Human", Some(2)), 4, false)
        .unwrap();
    let err = d.validate().unwrap_err().to_string();
    assert!(err.contains("il en faut 60"), "{err}");
}

#[test]
fn mana_cost_parse() {
    let c = ManaCost::parse("{2}{R}{R}");
    assert_eq!(c.cmc(), 4);
    assert_eq!(c.to_notation(), "{2}{R}{R}");
}

#[test]
fn curve_buckets() {
    let mut d = Deck::new("Feu", Format::Standard);
    d.recruit(stub("3", "Ash", "Creature", Some(9)), 2, false)
        .unwrap();
    let st = d.stats();
    assert_eq!(st.curve.get(7), 2);
}
