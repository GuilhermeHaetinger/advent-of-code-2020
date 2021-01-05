use {
    super::util::io,
    std::collections::{HashSet, VecDeque},
};

fn part1(input_file: &str) -> i64 {
    let mut players = get_players(input_file);

    while players.len() != 1 {
        round(&mut players);
        players = players.into_iter().filter(|v| !v.is_empty()).collect();
    }

    let winner = players[0].clone();
    let winner_len = winner.len();
    let res: usize = winner
        .into_iter()
        .enumerate()
        .map(|(id, card)| (winner_len - id) * card)
        .sum();
    println!("Day 22 (P1) = {}", res);
    res as i64
}

fn part2(input_file: &str) -> i64 {
    let players = get_players(input_file);

    let mut player1 = players[0].clone();
    let mut player2 = players[1].clone();

    rec_round(&mut player1, &mut player2);

    let winner = if player1.is_empty() { player2 } else { player1 };

    let winner_len = winner.len();
    let res: usize = winner
        .into_iter()
        .enumerate()
        .map(|(id, card)| (winner_len - id) * card)
        .sum();
    println!("Day 22 (P2) = {}", res);
    res as i64
}

fn get_players(input_file: &str) -> Vec<VecDeque<usize>> {
    let lines = io::lines_from_file(input_file);
    let chunks = lines.rsplit(|s| s.is_empty());
    chunks
        .map(|cards| {
            let mut queue = VecDeque::new();
            cards[1..].iter().for_each(|card| {
                let num = card.parse::<usize>().unwrap();
                queue.push_back(num);
            });
            queue
        })
        .collect()
}

fn round(players: &mut Vec<VecDeque<usize>>) {
    let mut top_cards: Vec<(usize, usize)> = players
        .iter_mut()
        .enumerate()
        .map(|(player, hand)| (player, hand.pop_front().unwrap()))
        .collect();

    top_cards.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    let (winner_id, _) = top_cards[0];
    top_cards.into_iter().for_each(|(_, card)| {
        players[winner_id].push_back(card);
    });
}

fn rec_round(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) -> usize {
    let mut states = HashSet::new();
    loop {
        let this_state = format!("{:?}", player1);
        if states.contains(&this_state) {
            return 1;
        }

        states.insert(this_state.clone());

        let first = player1.pop_front().unwrap();
        let second = player2.pop_front().unwrap();

        let mut winner: usize;
        if first > second {
            winner = 0;
        } else {
            winner = 1;
        }

        if first <= player1.len() && second <= player2.len() {
            winner = rec_round(
                &mut player1.clone().drain(..first).collect(),
                &mut player2.clone().drain(..second).collect(),
            );
        }

        if winner == 0 {
            player1.push_back(first);
            player1.push_back(second);
        } else {
            player2.push_back(second);
            player2.push_back(first);
        }

        if player1.is_empty() {
            return 1;
        } else if player2.is_empty() {
            return 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day22.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 32413);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 31596);
    }
}
