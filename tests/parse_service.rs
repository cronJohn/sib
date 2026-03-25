mod fixtures;
mod helpers;

use crate::{
    fixtures::notes::{NotesFixture, TEST_DIFFICULTIES},
    helpers::sort_notes,
};

#[test]
fn collect_notes_bulk_randomized_success() {
    let bulk_amount = 100;
    let malformed_amount = 2;

    let mut fx = NotesFixture::default().bulk_random(bulk_amount);

    for i in 0..malformed_amount {
        fx = fx.malformed(&format!("bad/broken{}.md", i))
    }

    let (_tmp, service) = fx.build();

    let notes = sort_notes(service.collect_notes());

    for note in notes.iter().filter(|n| n.metadata.is_some()) {
        let meta = note.metadata.as_ref().unwrap();

        assert!(
            TEST_DIFFICULTIES.contains(&meta.difficulty.as_deref().unwrap()),
            "Invalid difficulty parsed"
        );

        assert!(!meta.tags.is_empty(), "Tags should not be empty");

        assert!(
            note.content.contains("Random content"),
            "Content parsing failed"
        );
    }

    let total_expected = bulk_amount + malformed_amount;
    assert_eq!(notes.len(), total_expected);

    // Count metadata-bearing notes
    let with_metadata = notes.iter().filter(|n| n.metadata.is_some()).count();
    let without_metadata = notes.iter().filter(|n| n.metadata.is_none()).count();

    assert_eq!(
        with_metadata, bulk_amount,
        "Valid notes failed to parse metadata"
    );
    assert_eq!(
        without_metadata, malformed_amount,
        "Malformed notes were not handled correctly"
    );
}

#[test]
fn parse_single_note_exact_success() {
    let (_tmp, service) = NotesFixture::default()
        .file(
            "note.md",
            r#"---
difficulty: "easy"
tags: ["tag1", "tag2"]
---
Hello world"#,
        )
        .build();

    let notes = service.collect_notes();
    assert_eq!(notes.len(), 1);

    let note = &notes[0];

    let meta = note.metadata.as_ref().unwrap();
    assert_eq!(meta.difficulty.as_deref(), Some("easy"));
    assert_eq!(meta.tags, vec!["tag1", "tag2"]);

    assert_eq!(note.content.trim(), "Hello world");
}

#[test]
fn parse_partial_note_success() {
    let (_tmp, service) = NotesFixture::default().partial("note.md").build();

    let notes = service.collect_notes();
    assert_eq!(notes.len(), 1);

    let note = &notes[0];

    let meta = note.metadata.as_ref().unwrap();
    assert!(
        meta.difficulty.is_none(),
        "Difficulty metadata should be empty"
    );
}

#[test]
fn rejects_unclosed_frontmatter() {
    let (_tmp, service) = NotesFixture::default()
        .file(
            "broken.md",
            r#"---
difficulty: "easy"
tags: ["a", "b"]
# Missing closing ---
Hello world"#,
        )
        .build();

    let notes = service.collect_notes();
    let note = &notes[0];

    assert!(
        note.metadata.is_none(),
        "Should not parse incomplete frontmatter"
    );
}

/// Test race issues and fs bottlenecks
#[test]
fn large_scale_note_collection_success() {
    let (_tmp, service) = NotesFixture::default().bulk_random(1000).build();

    let notes = service.collect_notes();

    assert_eq!(notes.len(), 1000);
}
