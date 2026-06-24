use std::cmp::Ordering;

use agentx::{Flags, List, Num, Str, Text};

#[test]
fn parse_control_reads_action_and_note () {

    let ( action, note ) = Text::parse_control("ACTION: ship\nNOTE: looks good");

    assert_eq!(action, "ship");
    assert_eq!(note, "looks good");

    let ( verdict, _ ) = Text::parse_control("ACTION: revise\nfix the seam");

    assert_eq!(verdict, "revise");

    let ( empty, blank ) = Text::parse_control("no verdict line here");

    assert!(empty.is_empty());
    assert!(blank.is_empty());

}

#[test]
fn slug_lowercases_and_dashes () {

    assert_eq!(Text::slug("Hello, World! 2"), "hello-world-2");
    assert_eq!(Text::slug("  Already-Clean  "), "already-clean");
    assert!(Text::slug("@#$").is_empty());

}

#[test]
fn natural_compare_orders_numbers_numerically () {

    assert_eq!(Text::natural_compare("item2", "item10"), Ordering::Less);
    assert_eq!(Text::natural_compare("b", "a"), Ordering::Greater);
    assert_eq!(Text::natural_compare("Same", "same"), Ordering::Equal);

}

#[test]
fn line_helpers_skip_blank_edges () {

    let body = "\n  Title \n\nbody line\n\n";

    assert_eq!(Text::first_line(body), "Title");
    assert_eq!(Text::last_line(body), "body line");
    assert!(Text::last_line_is("a\nDONE\n", "done"));
    assert!(!Text::last_line_is("a\nDONE\n", "ready"));

}

#[test]
fn str_case_and_split () {

    assert_eq!(Str::upper("abc"), "ABC");
    assert_eq!(Str::lower("ABC"), "abc");
    assert_eq!(Str::capitalize("hello"), "Hello");
    assert_eq!(Str::split("a,b,c", ","), ["a", "b", "c"]);

}

#[test]
fn num_parse_and_clamp () {

    assert_eq!(Num::parse_int("42"), Some(42));
    assert_eq!(Num::parse_int("nope"), None);
    assert_eq!(Num::clamp(15, 0, 10), 10);
    assert_eq!(Num::clamp(-3, 0, 10), 0);
    assert!(Num::is_even(4));
    assert!(!Num::is_even(3));

}

#[test]
fn list_unique_sorted_contains () {

    let mut unique = List::unique(&[3, 1, 2, 3, 1]);
    unique.sort_unstable();

    assert_eq!(unique, [1, 2, 3]);
    assert_eq!(List::sorted(&[3, 1, 2]), [1, 2, 3]);
    assert!(List::contains(&[1, 2, 3], &2));
    assert!(!List::contains(&[1, 2, 3], &9));

}

#[test]
fn flags_default_is_empty () {

    let flags = Flags::default();

    assert!(flags.inspire.is_none());
    assert!(flags.gate.is_none());
    assert!(flags.tests.is_none());
    assert!(flags.ignore.is_empty());
    assert!(flags.include.is_empty());
    assert!(!flags.background);

}
