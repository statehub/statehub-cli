//
// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
// Use is subject to license terms.
//

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use console::Emoji;

const STATE_LABELS: &[Emoji<'static, 'static>] = &[
    Emoji("☘", "o"),
    Emoji("🦍", "o"),
    Emoji("🦧", "o"),
    Emoji("🐕", "o"),
    Emoji("🐺", "o"),
    Emoji("🦊", "o"),
    Emoji("🦝", "o"),
    Emoji("🐈", "o"),
    Emoji("🦁", "o"),
    Emoji("🦄", "o"),
    Emoji("🦓", "o"),
    Emoji("🦌", "o"),
    Emoji("🐏", "o"),
    Emoji("🐪", "o"),
    Emoji("🦙", "o"),
    Emoji("🦒", "o"),
    Emoji("🐘", "o"),
    Emoji("🦏", "o"),
    Emoji("🦛", "o"),
    Emoji("🐇", "o"),
    Emoji("🐿️", "o"),
    Emoji("🦫", "o"),
    Emoji("🦔", "o"),
    Emoji("🦘", "o"),
    Emoji("🦅", "o"),
    Emoji("🦉", "o"),
    Emoji("🦜", "o"),
    Emoji("🦖", "o"),
    Emoji("🐋", "o"),
    Emoji("🐬", "o"),
    Emoji("🐠", "o"),
    Emoji("🦈", "o"),
    Emoji("🐝", "o"),
    Emoji("🐞", "o"),
    Emoji("🦂", "o"),
];

pub(crate) fn get_label(text: &str) -> Emoji<'static, 'static> {
    let idx = hash(text) as usize % STATE_LABELS.len();
    STATE_LABELS[idx]
}

fn hash(text: &str) -> u64 {
    let mut hash = DefaultHasher::new();
    text.hash(&mut hash);
    hash.finish()
}
