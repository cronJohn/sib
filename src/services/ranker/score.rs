use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    domain::{note::Note, tokenizer::Token},
    services::{
        parse::NoteMetadataState,
        ranker::{
            RankerService, UsageStats,
            constants::{
                FREQUENCY_DAMPENING, FREQUENCY_OFFSET, FREQUENCY_WEIGHT, META_BOOST,
                RECENCY_TAU_DAYS, RECENCY_WEIGHT, SCALE, SLUG_BOOST, TAG_BOOST,
            },
        },
    },
};

const SECONDS_PER_DAY: f64 = 86400.0;

impl RankerService {
    pub fn score(&self, note: &Note, tokens: &[Token], usage: Option<&UsageStats>) -> i32 {
        let mut score = 0;

        for token in tokens {
            match token {
                Token::Tag(tag) => {
                    if !note
                        .metadata
                        .get_metadata()
                        .unwrap()
                        .tags
                        .iter()
                        .any(|t| t.contains(tag))
                    {
                        return 0; // hard filter
                    }
                    score += TAG_BOOST;
                }

                Token::Meta { key, value } => {
                    match &note.metadata {
                        NoteMetadataState::Valid(md) => {
                            if !md.get_as_string(key).is_some_and(|v| v.contains(value)) {
                                return 0; // hard filter
                            }
                            score += META_BOOST;
                        }
                        _ => return 0,
                    }
                }

                Token::Text(txt) => {
                    if note.slug.to_string_lossy().contains(txt) {
                        score += SLUG_BOOST;
                    }
                }
            }
        }

        if let Some((usage, last_opened)) = usage.and_then(|u| u.last_opened.map(|lo| (u, lo))) {
            let tau = RECENCY_TAU_DAYS * SECONDS_PER_DAY;

            let age_secs = (now_ts() - last_opened) as f64;

            // exponential decay
            let recency = (-(age_secs / tau)).exp() * RECENCY_WEIGHT;

            // dampened frequency
            let frequency = ((usage.open_count as f64 * FREQUENCY_DAMPENING) + FREQUENCY_OFFSET)
                .ln()
                * FREQUENCY_WEIGHT;

            // combined score
            let score_boost = recency.powf(2.0) * frequency;

            score += (score_boost * SCALE) as i32;
        }

        score
    }
}

pub fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use crate::test_utils::fixtures::{
        note::NoteFixture, token::TokenFixture, usage_stats::UsageStatsFixture,
    };

    use super::*;

    const MIN_AGO: u64 = 60;
    const HOUR_AGO: u64 = 60 * 60;
    const DAY_AGO: u64 = 60 * 60 * 12;

    #[test]
    fn tag_should_boost() {
        let ranker = RankerService::default();
        let note = NoteFixture::default().tag("rust").slug("slug").build();
        let tokens = TokenFixture::default().tag("rust").build();
        let score = ranker.score(&note, &tokens, None);
        let expected = TAG_BOOST;
        assert_eq!(
            score, expected,
            "Mismatched boost. Expected: {expected}, Actual: {score}"
        );
    }

    #[test]
    fn tag_missing_should_return_zero() {
        let ranker = RankerService::default();
        let note = NoteFixture::default().tag("rust").slug("slug").build();
        let tokens = TokenFixture::default().tag("python").build();
        let score = ranker.score(&note, &tokens, None);
        assert_eq!(score, 0, "Missing tag should hard filter to 0");
    }

    #[test]
    fn meta_should_boost() {
        let ranker = RankerService::default();
        let note = NoteFixture::default()
            .slug("note")
            .field("author", "alice")
            .build();
        let tokens = TokenFixture::default().meta("author", "alice").build();
        let score = ranker.score(&note, &tokens, None);
        let expected = META_BOOST;
        assert_eq!(
            score, expected,
            "Meta match should boost. Expected: {expected}, Actual: {score}"
        );
    }

    #[test]
    fn meta_missing_should_return_zero() {
        let ranker = RankerService::default();
        let note = NoteFixture::default()
            .slug("note")
            .field("author", "alice")
            .build();
        let tokens = TokenFixture::default().meta("author", "bob").build();
        let score = ranker.score(&note, &tokens, None);
        assert_eq!(score, 0, "Missing meta value should hard filter to 0");
    }

    #[test]
    fn text_in_slug_should_boost() {
        let ranker = RankerService::default();
        let note = NoteFixture::default().slug("my-rust-note").build();
        let tokens = TokenFixture::default().text("rust").build();
        let score = ranker.score(&note, &tokens, None);
        let expected = SLUG_BOOST;
        assert_eq!(
            score, expected,
            "Text in slug should boost. Expected: {expected}, Actual: {score}"
        );
    }

    #[test]
    fn text_not_in_slug_should_not_boost() {
        let ranker = RankerService::default();
        let note = NoteFixture::default().slug("my-note").build();
        let tokens = TokenFixture::default().text("rust").build();
        let score = ranker.score(&note, &tokens, None);
        let expected = 0;
        assert_eq!(
            score, expected,
            "Text not in slug should not boost. Expected: {expected}, Actual: {score}"
        );
    }

    #[test]
    fn more_recent_should_beat_less_recent() {
        let ranker = RankerService::default();
        let note = NoteFixture::default().build();
        let tokens = vec![];

        let now = now_ts();

        let fresh_used = UsageStatsFixture::default()
            .open_count(5)
            .last_opened(Some(now - MIN_AGO));

        let older_used = fresh_used.clone().last_opened(Some(now - DAY_AGO)).build();

        let recent = ranker.score(&note, &tokens, Some(&fresh_used.build()));
        let old = ranker.score(&note, &tokens, Some(&older_used));

        assert!(
            recent > old,
            "More recent files should beat less recent. Newer: {recent} should be more than older: {old}"
        );
    }

    #[test]
    fn recent_low_freq_beats_old_high_freq() {
        let ranker = RankerService::default();
        let note = NoteFixture::default().build();
        let tokens = vec![];

        let now = now_ts();

        let fresh_used = UsageStatsFixture::default()
            .last_opened(Some(now - MIN_AGO))
            .open_count(1)
            .build();

        let older_used = UsageStatsFixture::default()
            .last_opened(Some(now - DAY_AGO * 7))
            .open_count(100)
            .build();

        let recent = ranker.score(&note, &tokens, Some(&fresh_used));
        let old = ranker.score(&note, &tokens, Some(&older_used));

        assert!(
            recent > old,
            "Fresh less opened should be prioritized over older more opened. Newer: {recent}, Older: {old}"
        );
    }

    #[test]
    fn higher_freq_wins_when_recency_equal() {
        let ranker = RankerService::default();
        let note = NoteFixture::default().build();
        let tokens = vec![];

        let now = now_ts();

        let less_used = UsageStatsFixture::default()
            .last_opened(Some(now - HOUR_AGO))
            .open_count(1);

        let more_used = less_used.clone().open_count(10).build();

        let low = ranker.score(&note, &tokens, Some(&less_used.build()));
        let high = ranker.score(&note, &tokens, Some(&more_used));

        assert!(
            high > low,
            "More opens should beat less opens when recency is equal: More opens: {high}, Less opens: {low}"
        );
    }

    #[test]
    fn score_decreases_as_age_increases() {
        let ranker = RankerService::default();
        let note = NoteFixture::default().build();
        let tokens = vec![];

        let now = now_ts();

        let one_min_used = UsageStatsFixture::default()
            .open_count(5)
            .last_opened(Some(now - MIN_AGO));

        let one_hour_used = one_min_used
            .clone()
            .last_opened(Some(now - HOUR_AGO))
            .build();

        let one_day_used = one_min_used
            .clone()
            .last_opened(Some(now - DAY_AGO))
            .build();

        let s1 = ranker.score(&note, &tokens, Some(&one_min_used.build()));
        let s2 = ranker.score(&note, &tokens, Some(&one_hour_used));
        let s3 = ranker.score(&note, &tokens, Some(&one_day_used));

        assert!(
            s1 > s2 && s2 > s3,
            "Score should decrease as age increases: 1m: {s1}, 1h: {s2}, 1d: {s3}",
        );
    }

    #[test]
    fn ranking_behaves_reasonably() {
        let ranker = RankerService::default();
        let note = NoteFixture::default().build();
        let tokens = vec![];

        let now = now_ts();

        let items = vec![
            (60, 1),         // very recent
            (86400, 10),     // recent + frequent
            (86400 * 3, 50), // older but frequent
        ];

        let mut scored: Vec<_> = items
            .into_iter()
            .map(|(age, freq)| {
                (
                    age,
                    freq,
                    ranker.score(
                        &note,
                        &tokens,
                        Some(
                            &UsageStatsFixture::default()
                                .last_opened(Some(now - age))
                                .open_count(freq)
                                .build(),
                        ),
                    ),
                )
            })
            .collect();

        scored.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        // sanity expectations
        assert_eq!(scored[0].0, 60); // most recent should win
    }
}
