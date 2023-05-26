use strum::IntoEnumIterator;

use crate::card::{ Rank, Suit };
use crate::hand::Hand;


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_rank_score() {
        // check non-empty list of ranks
        let ranks = vec![Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace, Rank::King];
        assert_eq!(calculate_rank_score(ranks), 978669);
        
        let ranks = vec![Rank::Ace, Rank::Ace, Rank::Ace, Rank::Ace, Rank::Queen];
        assert_eq!(calculate_rank_score(ranks), 978668);

        let ranks = vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten];
        assert_eq!(calculate_rank_score(ranks), 974010);

        let ranks = vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Nine];
        assert_eq!(calculate_rank_score(ranks), 974009);

        let score = calculate_rank_score(vec![Rank::Ace, Rank::King, Rank::Queen]);
        assert_eq!(score, 0b1110_1101_1100);

        let score = calculate_rank_score(vec![Rank::Two, Rank::Three, Rank::Four]);
        assert_eq!(score, 0b0010_0011_0100);

        let score = calculate_rank_score(vec![Rank::Ten, Rank::Nine, Rank::Eight]);
        assert_eq!(score, 0b1010_1001_1000);

        // check single rank
        let score = calculate_rank_score(vec![Rank::Ace]);
        assert_eq!(score, 14);
        
        // check ranks out of order
        let score = calculate_rank_score(vec![Rank::Two, Rank::Ace, Rank::Three]);
        assert_eq!(score, 0b0010_1110_0011);
        
        // check with duplicates
        let score = calculate_rank_score(vec![Rank::Ace, Rank::Ace, Rank::King]);
        assert_eq!(score, 0b1110_1110_1101);
    }

    #[test]
    fn test_calculate_rank_score_empty() {
        // check empty list of ranks
        let result = calculate_rank_score(vec![]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_evaluate_flush() {
        let hand = Hand::new_from_str("2s 4s 6s 8s Ts Ks Qs").unwrap();
        let flush_score = evaluate_flush(hand);
        let expected_score = 5000000 + (13 << 16) + (12 << 12) + (10 << 8) + (8 << 4) + 6;
        assert_eq!(flush_score, expected_score);

        let hand = Hand::new_from_str("2s 2h 6s 8s Ts Ks Kd").unwrap();
        let flush_score = evaluate_flush(hand);
        let expected_score = 5000000 + (13 << 16) + (10 << 12) + (8 << 8) + (6 << 4) + 2;
        assert_eq!(flush_score, expected_score);

        let hand = Hand::new_from_str("2s 4s 6s 8s Ts Kh Ad").unwrap();
        let flush_score = evaluate_flush(hand);
        let expected_score = 5000000 + (10 << 16) + (8 << 12) + (6 << 8) + (4 << 4) + 2;
        assert_eq!(flush_score, expected_score);

        let hand = Hand::new_from_str("As 2s 3s 4s 5s Kd Qd").unwrap();
        let flush_score = evaluate_flush(hand);
        let expected_score = 5000000 + (14 << 16) + (5 << 12) + (4 << 8) + (3 << 4) + 2;
        assert_eq!(flush_score, expected_score);

        let hand = Hand::new_from_str("2s 4s 6s 8d Td Kd Qs").unwrap();
        let flush_score = evaluate_flush(hand);
        assert_eq!(flush_score, 0);

        let hand = Hand::new_from_str("2s 4s 6s 8s").unwrap();
        let flush_score = evaluate_flush(hand);
        assert_eq!(flush_score, 0);
    }

    #[test]
    fn test_get_flush_ranks_with_flush() {
        let hand = Hand::new_from_str("2s 4s 6s 8s Ts Ks Qs").unwrap();
        let flush_ranks = get_flush_ranks_desc(hand);
        assert_eq!(flush_ranks, vec![Rank::King, Rank::Queen, Rank::Ten, Rank::Eight, Rank::Six, Rank::Four, Rank::Two]);

        let hand = Hand::new_from_str("2s 4s 6s 8s Ts Ks Qc").unwrap();
        let flush_ranks = get_flush_ranks_desc(hand);
        assert_eq!(flush_ranks, vec![Rank::King, Rank::Ten, Rank::Eight, Rank::Six, Rank::Four, Rank::Two]);

        let hand = Hand::new_from_str("2s 4s 6s 8s Ts Ks Kc").unwrap();
        let flush_ranks = get_flush_ranks_desc(hand);
        assert_eq!(flush_ranks, vec![Rank::King, Rank::Ten, Rank::Eight, Rank::Six, Rank::Four, Rank::Two]);

    }

    #[test]
    fn test_get_flush_ranks_without_flush() {
        let hand = Hand::new_from_str("2s 4s 6s 8d Td Kd Qs").unwrap();
        assert!(get_flush_ranks_desc(hand).is_empty());
    }

    #[test]
    fn test_is_ace_low_straight_true() {
        let hand = Hand::new_from_str("Ac 2h 3d 4s 5c").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(is_ace_low_straight(&ranks_desc));

        let hand = Hand::new_from_str("Ac 5h 2d 4s 3c").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(is_ace_low_straight(&ranks_desc));

        let hand = Hand::new_from_str("Ac 2h 3d 4s 5c 6s 7d").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(is_ace_low_straight(&ranks_desc));

        let hand = Hand::new_from_str("Ac 5h 2d 4s 3c Ad 3s").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(is_ace_low_straight(&ranks_desc));

    }

    #[test]
    fn test_is_ace_low_straight_false() {
        let hand = Hand::new_from_str("2c 3h 4d 5s 6c").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(!is_ace_low_straight(&ranks_desc));

        let hand = Hand::new_from_str("Ac 2h 3d 4s").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(!is_ace_low_straight(&ranks_desc));

        let hand = Hand::new_from_str("Ac 2h 3d 4s 5c 6s").unwrap();
        let ranks_desc = hand.get_ranks();
        assert!(is_ace_low_straight(&ranks_desc));
    }
}
