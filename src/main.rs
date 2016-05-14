extern crate rand;

use rand::thread_rng;
use rand::Rng;

const CARDS: usize = 13;

#[inline]
fn get_random(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(min, max) + min
}

fn init(mut count: usize) -> Vec<usize> {
    let mut cards = Vec::new();
    while count > 0 {
        cards.push(count);
        count-=1;
    }
    cards
}

fn draw_one(cards: &mut Vec<usize>) -> usize {
    let len = cards.len(); //stupid borrow checker
    cards.remove(get_random(0, len))
}

fn shuffle(cards: &mut Vec<usize>) -> Vec<usize> {
    let mut deck = cards.clone();
    let mut ret = Vec::new();
    while deck.len() > 0 {
        ret.push(draw_one(&mut deck));
    }
    ret
}
#[cfg(not(feature="parallel"))]
fn main() {
    let mut cards = init(CARDS);
    let mut results = vec![shuffle(&mut cards)];
    'main: loop {
        let result = shuffle(&mut cards);
        for i in 0..results.len() {
            if result == results[i] {
                println!("yay! {:?} == {:?}", results[i], result);
                results.push(result.clone());
                break 'main;
            }
        }
        results.push(result);
    }
    println!("The computer has shuffled {} times to get the same order of the deck that has appeared before when there are {} cards on the deck ", results.len(), CARDS);
}
