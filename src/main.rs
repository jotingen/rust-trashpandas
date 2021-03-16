#[derive(Debug)]
#[derive(Default)]
struct Card {
    name: String,
    points_for_first: u16,
    points_for_second: u16,
    points_for_third: u16,
    points_for_each_card: u16,
    roll_end: bool,
    roll_continue: bool,
    stop_steal_draw_two: bool,
    stop_steal_steal_one: bool,
    use_double_token: bool,
    use_force_continue: bool,
    use_take_from_discard: bool,
    use_steal_from_stash: bool,
    description: String,
}

fn main() {
    println!("Hello, world!");

    let mut deck = Vec::new();

    for _i in 0..13 {
        deck.push(Card {
            name: "BLAMMO!".to_string(),
            points_for_each_card: 1,
            roll_continue: true,
            description: "Re-roll your last die. 1 point per BLAMMO! stashed.".to_string(),
            ..Default::default()
        });
    }

    for _i in 0..11 {
        deck.push(Card {
            name: "NANNERS".to_string(),
            points_for_first: 7,
            roll_end: true,
            description: "Ignore your last roll and choose to stop.".to_string(),
            ..Default::default()
        });
    }

    for _i in 0..9 {
        deck.push(Card {
            name: "MMM PIE".to_string(),
            points_for_first: 6,
            points_for_second: 2,
            points_for_third: 1,
            use_double_token: true,
            description: "Resolve a token twice".to_string(),
            ..Default::default()
        });
    }

    for _i in 0..5 {
        deck.push(Card {
            name: "YUM YUM".to_string(),
            points_for_first: 4,
            points_for_second: 2,
            points_for_third: 0,
            use_force_continue: true,
            description: "Force a player to continue rolling.".to_string(),
            ..Default::default()
        });
    }

    for _i in 0..7 {
        deck.push(Card {
            name: "FEESH".to_string(),
            points_for_first: 5,
            points_for_second: 3,
            points_for_third: 1,
            use_take_from_discard: true,
            description: "Force a player to continue rolling.".to_string(),
            ..Default::default()
        });
    }

    for _i in 0..2 {
        deck.push(Card {
            name: "KITTEH".to_string(),
            stop_steal_steal_one: true,
            description: "If a player tries to steal from you, steal from them instead.".to_string(),
            ..Default::default()
        });
    }

    for _i in 0..3 {
        deck.push(Card {
            name: "SHINY".to_string(),
            points_for_first: 3,
            use_steal_from_stash: true,
            description: "Steal a stashed card form a player. (Put in your hand)".to_string(),
            ..Default::default()
        });

    for _i in 0..3 {
        deck.push(Card {
            name: "DOGGO".to_string(),
            stop_steal_draw_two: true,
            description: "Block a steal attempt, then draw 2 cards.".to_string(),
            ..Default::default()
        });
    }

    }

    println!("deck : ");
    for card in deck {
        println!("  {:?}", card);
    }
}
