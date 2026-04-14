use sib::{
    domain::note::NoteMetadataBuilder,
    services::parse::NoteMetadataState,
    test_utils::{
        fixtures::note_env::{NoteEnvFixture, TEST_DIFFICULTIES},
        helpers::sort_notes,
    },
};

#[test]
fn collect_notes_bulk_randomized_success() {
    let bulk_amount = 100;
    let malformed_amount = 2;

    let mut fx = NoteEnvFixture::default().bulk_random(bulk_amount);

    for i in 0..malformed_amount {
        fx = fx.malformed(&format!("bad/broken{}.md", i));
    }

    let (_tmp, service) = fx.build();
    let notes = sort_notes(service.collect_notes());

    assert_eq!(notes.len(), bulk_amount + malformed_amount);

    let mut valid_count = 0;
    let mut invalid_count = 0;
    let mut none_count = 0;

    for note in &notes {
        match &note.metadata {
            NoteMetadataState::Valid(meta) => {
                valid_count += 1;

                // Tags should never be empty
                assert!(!meta.tags.is_empty(), "Tags should not be empty");

                // Content should contain the random note marker
                assert!(
                    note.content.contains("Random content"),
                    "Content parsing failed"
                );

                // Difficulty must exist and be one of the allowed values
                if let Some(value) = meta.extra.get("difficulty") {
                    assert!(
                        TEST_DIFFICULTIES.contains(&value.as_str()),
                        "Difficulty {:?} not in allowed set",
                        value
                    );
                } else {
                    panic!("Difficulty metadata missing");
                }
            }

            NoteMetadataState::Invalid(_) => {
                invalid_count += 1;
            }

            NoteMetadataState::None => {
                none_count += 1;
            }
        }
    }

    assert_eq!(valid_count, bulk_amount, "Valid notes failed to parse");
    assert_eq!(
        invalid_count, malformed_amount,
        "Malformed notes not detected"
    );
    assert_eq!(none_count, 0, "Unexpected notes without metadata");
}

#[test]
fn parse_single_note_exact_success() {
    let expected_note_slug = "note.md";
    let metadata = NoteMetadataBuilder::default()
        .tag("tag1")
        .tag("tag2")
        .field("difficulty", "easy")
        .build();

    let (_tmp, service) = NoteEnvFixture::default()
        .file(expected_note_slug, Some(metadata.clone()), "Hello world")
        .build();

    let notes = service.collect_notes();
    assert_eq!(notes.len(), 1);

    let note = &notes[0];

    match &note.metadata {
        NoteMetadataState::Valid(meta) => {
            assert_eq!(meta.tags, vec!["tag1", "tag2"]);

            assert_eq!(meta.extra.get("difficulty").unwrap(), "easy");
        }
        _ => panic!("Expected valid metadata"),
    }

    assert_eq!(note.content.trim(), "Hello world");
    assert_eq!(note.slug.to_str().unwrap(), expected_note_slug);
}

#[test]
fn parse_partial_note_success() {
    let metadata = NoteMetadataBuilder::default().tag("only-tags").build();

    let (_tmp, service) = NoteEnvFixture::default()
        .file("note.md", Some(metadata), "Partial metadata")
        .build();

    let notes = service.collect_notes();
    let note = &notes[0];

    match &note.metadata {
        NoteMetadataState::Valid(meta) => {
            assert_eq!(meta.tags, vec!["only-tags"]);
            assert!(meta.extra.is_empty());
        }
        _ => panic!("Expected valid metadata"),
    }
}

#[test]
fn rejects_unclosed_frontmatter() {
    let (_tmp, service) = NoteEnvFixture::default()
        .raw(
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
        matches!(note.metadata, NoteMetadataState::Invalid(_)),
        "Should detect invalid frontmatter"
    );
}

#[test]
fn malformed_yaml_is_detected() {
    let (_tmp, service) = NoteEnvFixture::default().malformed("bad.md").build();

    let notes = service.collect_notes();
    let note = &notes[0];

    assert!(matches!(note.metadata, NoteMetadataState::Invalid(_)));
}

/// Test race issues and fs bottlenecks
#[test]
fn large_scale_note_collection_success() {
    let (_tmp, service) = NoteEnvFixture::default().bulk_random(1000).build();

    let notes = service.collect_notes();

    assert_eq!(notes.len(), 1000);
}
