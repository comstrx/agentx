use std::cmp::Ordering;

use agentx::core::fs::Dir;
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
    assert!(flags.description.is_none());
    assert!(flags.gate.is_none());
    assert!(flags.audits.is_none());
    assert!(flags.tests.is_none());
    assert!(flags.doc_blocks.is_none());
    assert!(flags.doc_contracts.is_none());
    assert!(flags.ignore.is_empty());
    assert!(flags.include.is_empty());
    assert!(!flags.background);

}

#[test]
fn unprefix_strips_one_leading_number_segment () {

    assert_eq!(Text::unprefix("01_foo"), "foo");
    assert_eq!(Text::unprefix("1_foo"), "foo");
    assert_eq!(Text::unprefix("12_next-saas-admin-panel"), "next-saas-admin-panel");
    assert_eq!(Text::unprefix("foo"), "foo");
    assert_eq!(Text::unprefix("_foo"), "_foo");
    assert_eq!(Text::unprefix("foo_1"), "foo_1");
    assert_eq!(Text::unprefix(""), "");

}

#[test]
fn stripped_inspire_matching_is_prefix_and_case_stable () {

    let a = Text::slug(Text::unprefix("01_laravel-saas-tenancy-api"));
    let b = Text::slug(Text::unprefix("Laravel-SaaS-Tenancy-API"));
    let c = Text::slug(Text::unprefix("laravel-saas-tenancy-api"));

    assert_eq!(a, c);
    assert_eq!(a, b);

}

#[test]
fn locate_matches_stripped_name_case_insensitively_last_wins () {

    let base = std::env::temp_dir().join(format!("agentx-locate-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);

    for child in ["01_rust", "02_php", "07_rust"] {

        std::fs::create_dir_all(base.join(child)).unwrap();

    }

    assert_eq!(Dir::locate(&base, "php").file_name().unwrap(), "02_php");
    assert_eq!(Dir::locate(&base, "PHP").file_name().unwrap(), "02_php");
    assert_eq!(Dir::locate(&base, "05_php").file_name().unwrap(), "02_php");
    assert_eq!(Dir::locate(&base, "rust").file_name().unwrap(), "07_rust");
    assert_eq!(Dir::locate(&base, "go"), base.join("go"));

    let _ = std::fs::remove_dir_all(&base);

}
