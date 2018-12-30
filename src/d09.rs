pub fn max_score(players: u32, max_value: u32) -> u32 {
    *MarbleGame::new(players as usize, max_value)
        .play()
        .iter()
        .max()
        .unwrap()
}

struct MarbleGame {
    marbles: Vec<u32>,
    scores: Vec<u32>,
    current_marble_index: usize,
    current_player_index: usize,
    current_value: u32,
    max_value: u32,
}

impl MarbleGame {
    fn new(player_count: usize, max_value: u32) -> MarbleGame {
        let capacity = (max_value + 1) - 2 * ((max_value + 1) / 23);
        let mut marbles = Vec::with_capacity(capacity as usize);
        marbles.push(0);
        let scores = vec![0; player_count];
        let current_marble_index = 0;
        let current_player_index = 0;
        let current_value = 0;

        MarbleGame {
            marbles,
            scores,
            current_marble_index,
            current_player_index,
            current_value,
            max_value,
        }
    }

    fn play(&mut self) -> &[u32] {
        while self.insert_marble() {}
        &self.scores
    }

    fn insert_marble(&mut self) -> bool {
        if self.current_value == self.max_value {
            return false;
        }

        self.current_value += 1;
        if self.current_value % 23 != 0 {
            self.current_marble_index = self.next_marble_index();
            self.marbles
                .insert(self.current_marble_index, self.current_value);
        } else {
            self.current_marble_index = self.remove_marble_index();
            self.scores[self.current_player_index] +=
                self.current_value + self.marbles.remove(self.current_marble_index);
            self.current_marble_index %= self.marbles.len();
        }

        self.current_player_index = self.next_player_index();
        true
    }

    fn next_marble_index(&self) -> usize {
        let index = self.current_marble_index + 1;
        index % self.marbles.len() + 1
    }

    fn remove_marble_index(&self) -> usize {
        let len = self.marbles.len() as isize;
        let mut index = (self.current_marble_index as isize - 7) % len;
        if index < 0 {
            index += len;
        }
        index as usize
    }

    fn next_player_index(&self) -> usize {
        let index = self.current_player_index + 1;
        index % self.scores.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_score() {
        assert_eq!(32, max_score(9, 25));
        assert_eq!(8317, max_score(10, 1618));
        assert_eq!(146373, max_score(13, 7999));
        assert_eq!(2764, max_score(17, 1104));
        assert_eq!(54718, max_score(21, 6111));
        assert_eq!(37305, max_score(30, 5807));
    }

    #[test]
    fn test_marbles() {
        let mut game = MarbleGame::new(9, 25);
        assert_eq!(game.marbles, vec![0]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 1]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 2, 1]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 2, 1, 3]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 4, 2, 1, 3]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 4, 2, 5, 1, 3]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 4, 2, 5, 1, 6, 3]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 4, 2, 5, 1, 6, 3, 7]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 8, 4, 2, 5, 1, 6, 3, 7]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 8, 4, 9, 2, 5, 1, 6, 3, 7]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 8, 4, 9, 2, 10, 5, 1, 6, 3, 7]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 6, 3, 7]);
        assert!(game.insert_marble());
        assert_eq!(game.marbles, vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 3, 7]);
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 7]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 16, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 16, 8, 17, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 16, 8, 17, 4, 18, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 16, 8, 17, 4, 18, 9, 19, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 16, 8, 17, 4, 18, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15]
        );
        assert!(game.insert_marble());
        assert_eq!(
            game.marbles,
            vec![
                0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 25, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7,
                15
            ]
        );
    }
}
