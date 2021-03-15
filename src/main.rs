#[derive(Debug)]
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
}

fn main() {
    println!("Hello, world!");

    let mut deck = Vec::new();

    for _i in 0..13 {
        deck.push(Card {
            name: "Banana".to_string(),
            points_for_first: 7,
            points_for_second: 0,
            points_for_third: 0,
            points_for_each_card: 0,
            roll_end: true,
            roll_continue: false,
            stop_steal_draw_two: false,
            stop_steal_steal_one: false,
        });
    }

    for _i in 0..13 {
        deck.push(Card {
            name: "Blammo".to_string(),
            points_for_first: 0,
            points_for_second: 0,
            points_for_third: 0,
            points_for_each_card: 1,
            roll_end: false,
            roll_continue: true,
            stop_steal_draw_two: false,
            stop_steal_steal_one: false,
        });
    }

    println!("deck : ");
    for card in deck {
        println!("  {:?}", card);
    }
}
