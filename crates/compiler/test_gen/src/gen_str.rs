#![cfg(not(feature = "gen-wasm"))]

#[cfg(feature = "gen-llvm")]
use crate::helpers::llvm::assert_evals_to;
#[cfg(feature = "gen-llvm")]
use crate::helpers::llvm::assert_llvm_evals_to;

#[cfg(feature = "gen-dev")]
use crate::helpers::dev::assert_evals_to;
#[cfg(feature = "gen-dev")]
use crate::helpers::dev::assert_evals_to as assert_llvm_evals_to;

#[allow(unused_imports)]
use indoc::indoc;
#[allow(unused_imports)]
use broc_std::{BrocList, BrocResult, BrocStr};

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn string_eq() {
    // context: the dev backend did not correctly mask the boolean that zig returns here
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [main] to "./platform"
            main : I64
            main = if "*" == "*" then 123 else 456
            "#
        ),
        123,
        u64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn string_neq() {
    // context: the dev backend did not correctly mask the boolean that zig returns here
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [main] to "./platform"
            main : I64
            main = if "*" != "*" then 123 else 456
            "#
        ),
        456,
        u64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_empty_delimiter() {
    assert_evals_to!(
        indoc!(
            r#"
                    List.len (Str.split "hello" "")
                "#
        ),
        1,
        usize
    );

    assert_evals_to!(
        indoc!(
            r#"
                    when List.first (Str.split "JJJ" "") is
                        Ok str ->
                            Str.countGraphemes str

                        _ ->
                            1729

                "#
        ),
        3,
        usize
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_bigger_delimiter_small_str() {
    assert_evals_to!(
        indoc!(
            r#"
                    List.len (Str.split "hello" "JJJJ there")
                "#
        ),
        1,
        usize
    );

    assert_evals_to!(
        indoc!(
            r#"
                    when List.first (Str.split "JJJ" "JJJJ there") is
                        Ok str ->
                            Str.countGraphemes str

                        _ ->
                            1729

                "#
        ),
        3,
        usize
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_str_concat_repeated() {
    assert_evals_to!(
        indoc!(
            r#"
                    when List.first (Str.split "JJJJJ" "JJJJ there") is
                        Ok str ->
                            str
                                |> Str.concat str
                                |> Str.concat str
                                |> Str.concat str
                                |> Str.concat str

                        _ ->
                            "Not Str!"

                "#
        ),
        BrocStr::from("JJJJJJJJJJJJJJJJJJJJJJJJJ"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_small_str_bigger_delimiter() {
    assert_evals_to!(
        indoc!(r#"Str.split "JJJ" "0123456789abcdefghi""#),
        BrocList::from_slice(&[BrocStr::from("JJJ")]),
        BrocList<BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_big_str_small_delimiter() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split "01234567789abcdefghi?01234567789abcdefghi" "?"
                "#
        ),
        BrocList::from_slice(&[
            BrocStr::from("01234567789abcdefghi"),
            BrocStr::from("01234567789abcdefghi")
        ]),
        BrocList<BrocStr>
    );

    assert_evals_to!(
        indoc!(
            r#"
                    Str.split "01234567789abcdefghi 3ch 01234567789abcdefghi" "3ch"
                "#
        ),
        BrocList::from_slice(&[
            BrocStr::from("01234567789abcdefghi "),
            BrocStr::from(" 01234567789abcdefghi")
        ]),
        BrocList<BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_small_str_small_delimiter() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split "J!J!J" "!"
                "#
        ),
        BrocList::from_slice(&[BrocStr::from("J"), BrocStr::from("J"), BrocStr::from("J")]),
        BrocList<BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_bigger_delimiter_big_strs() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split
                        "string to split is shorter"
                        "than the delimiter which happens to be very very long"
                "#
        ),
        BrocList::from_slice(&[BrocStr::from("string to split is shorter")]),
        BrocList<BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_empty_strs() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split "" ""
                "#
        ),
        BrocList::from_slice(&[BrocStr::from("")]),
        BrocList<BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_minimal_example() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split "a," ","
                "#
        ),
        BrocList::from_slice(&[BrocStr::from("a"), BrocStr::from("")]),
        BrocList<BrocStr>
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_small_str_big_delimiter() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split
                        "1---- ---- ---- ---- ----2---- ---- ---- ---- ----"
                        "---- ---- ---- ---- ----"
                        |> List.len
                "#
        ),
        3,
        usize
    );

    assert_evals_to!(
        indoc!(
            r#"
                    Str.split
                        "1---- ---- ---- ---- ----2---- ---- ---- ---- ----"
                        "---- ---- ---- ---- ----"
                "#
        ),
        BrocList::from_slice(&[BrocStr::from("1"), BrocStr::from("2"), BrocStr::from("")]),
        BrocList<BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_small_str_20_char_delimiter() {
    assert_evals_to!(
        indoc!(
            r#"
                    Str.split
                        "3|-- -- -- -- -- -- |4|-- -- -- -- -- -- |"
                        "|-- -- -- -- -- -- |"
                "#
        ),
        BrocList::from_slice(&[BrocStr::from("3"), BrocStr::from("4"), BrocStr::from("")]),
        BrocList<BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_concat_big_to_big() {
    assert_evals_to!(
            indoc!(
                r#"
                    Str.concat
                        "First string that is fairly long. Longer strings make for different errors. "
                        "Second string that is also fairly long. Two long strings test things that might not appear with short strings."
                "#
            ),
            BrocStr::from("First string that is fairly long. Longer strings make for different errors. Second string that is also fairly long. Two long strings test things that might not appear with short strings."),
            BrocStr
        );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn small_str_literal() {
    assert_llvm_evals_to!(
        "\"JJJJJJJJJJJJJJJJJJJJJJJ\"",
        [
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            0b1000_0000 | 23
        ],
        [u8; 24]
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn small_str_zeroed_literal() {
    // Verifies that we zero out unused bytes in the string.
    // This is important so that string equality tests don't randomly
    // fail due to unused memory being there!
    assert_llvm_evals_to!(
        "\"J\"",
        [
            0x4a,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0b1000_0001
        ],
        [u8; 24]
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn small_str_concat_empty_first_arg() {
    assert_llvm_evals_to!(
        r#"Str.concat "" "JJJJJJJJJJJJJJJ""#,
        [
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0b1000_0000 | 15
        ],
        [u8; 24]
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn small_str_concat_empty_second_arg() {
    assert_llvm_evals_to!(
        r#"Str.concat "JJJJJJJJJJJJJJJ" """#,
        [
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0b1000_0000 | 15
        ],
        [u8; 24]
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn small_str_concat_small_to_big() {
    assert_evals_to!(
        r#"Str.concat "abc" " this is longer than 15 chars""#,
        BrocStr::from("abc this is longer than 15 chars"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn small_str_concat_small_to_small_staying_small() {
    assert_llvm_evals_to!(
        r#"Str.concat "J" "JJJJJJJJJJJJJJ""#,
        [
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            b'J',
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0b1000_0000 | 15
        ],
        [u8; 24]
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn small_str_concat_small_to_small_overflow_to_big() {
    assert_evals_to!(
        r#"Str.concat "abcdefghijklm" "nopqrstuvwxyz""#,
        BrocStr::from("abcdefghijklmnopqrstuvwxyz"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_concat_empty() {
    assert_evals_to!(r#"Str.concat "" """#, BrocStr::default(), BrocStr);
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn small_str_is_empty() {
    assert_evals_to!(r#"Str.isEmpty "abc""#, false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn big_str_is_empty() {
    assert_evals_to!(
        r#"Str.isEmpty "this is more than 15 chars long""#,
        false,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn empty_str_is_empty() {
    assert_evals_to!(r#"Str.isEmpty """#, true, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_starts_with() {
    assert_evals_to!(r#"Str.startsWith "hello world" "hell""#, true, bool);
    assert_evals_to!(r#"Str.startsWith "hello world" """#, true, bool);
    assert_evals_to!(r#"Str.startsWith "nope" "hello world""#, false, bool);
    assert_evals_to!(r#"Str.startsWith "hell" "hello world""#, false, bool);
    assert_evals_to!(r#"Str.startsWith "" "hello world""#, false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_starts_with_scalar() {
    assert_evals_to!(
        &format!(r#"Str.startsWithScalar "foobar" {}"#, 'f' as u32),
        true,
        bool
    );
    assert_evals_to!(
        &format!(r#"Str.startsWithScalar "zoobar" {}"#, 'f' as u32),
        false,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_ends_with() {
    assert_evals_to!(r#"Str.endsWith "hello world" "world""#, true, bool);
    assert_evals_to!(r#"Str.endsWith "nope" "hello world""#, false, bool);
    assert_evals_to!(r#"Str.endsWith "" "hello world""#, false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_count_graphemes_small_str() {
    assert_evals_to!(r#"Str.countGraphemes "å🤔""#, 2, usize);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_count_graphemes_three_js() {
    assert_evals_to!(r#"Str.countGraphemes "JJJ""#, 3, usize);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_count_graphemes_big_str() {
    assert_evals_to!(
        r#"Str.countGraphemes "6🤔å🤔e¥🤔çppkd🙃1jdal🦯asdfa∆ltråø˚waia8918.,🏅jjc""#,
        45,
        usize
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_starts_with_same_big_str() {
    assert_evals_to!(
        r#"Str.startsWith "123456789123456789" "123456789123456789""#,
        true,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_starts_with_different_big_str() {
    assert_evals_to!(
        r#"Str.startsWith "12345678912345678910" "123456789123456789""#,
        true,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_starts_with_same_small_str() {
    assert_evals_to!(r#"Str.startsWith "1234" "1234""#, true, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_starts_with_different_small_str() {
    assert_evals_to!(r#"Str.startsWith "1234" "12""#, true, bool);
}
#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_starts_with_false_small_str() {
    assert_evals_to!(r#"Str.startsWith "1234" "23""#, false, bool);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_pass_single_ascii() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [97] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        broc_std::BrocStr::from("a"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_pass_many_ascii() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [97, 98, 99, 0x7E] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        broc_std::BrocStr::from("abc~"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_pass_single_unicode() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [0xE2, 0x88, 0x86] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        broc_std::BrocStr::from("∆"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_pass_many_unicode() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [0xE2, 0x88, 0x86, 0xC5, 0x93, 0xC2, 0xAC] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        broc_std::BrocStr::from("∆œ¬"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_pass_single_grapheme() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [0xF0, 0x9F, 0x92, 0x96] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        broc_std::BrocStr::from("💖"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_pass_many_grapheme() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [0xF0, 0x9F, 0x92, 0x96, 0xF0, 0x9F, 0xA4, 0xA0, 0xF0, 0x9F, 0x9A, 0x80] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        broc_std::BrocStr::from("💖🤠🚀"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_pass_all() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [0xF0, 0x9F, 0x92, 0x96, 98, 0xE2, 0x88, 0x86] is
                        Ok val -> val
                        Err _ -> ""
                "#
        ),
        broc_std::BrocStr::from("💖b∆"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_fail_invalid_start_byte() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [97, 98, 0x80, 99] is
                        Err (BadUtf8 InvalidStartByte byteIndex) ->
                            if byteIndex == 2 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        broc_std::BrocStr::from("a"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_fail_unexpected_end_of_sequence() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [97, 98, 99, 0xC2] is
                        Err (BadUtf8 UnexpectedEndOfSequence byteIndex) ->
                            if byteIndex == 3 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        broc_std::BrocStr::from("a"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_fail_expected_continuation() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [97, 98, 99, 0xC2, 0x00] is
                        Err (BadUtf8 ExpectedContinuation byteIndex) ->
                            if byteIndex == 3 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        broc_std::BrocStr::from("a"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_fail_overlong_encoding() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [97, 0xF0, 0x80, 0x80, 0x80] is
                        Err (BadUtf8 OverlongEncoding byteIndex) ->
                            if byteIndex == 1 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        broc_std::BrocStr::from("a"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_fail_codepoint_too_large() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [97, 0xF4, 0x90, 0x80, 0x80] is
                        Err (BadUtf8 CodepointTooLarge byteIndex) ->
                            if byteIndex == 1 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        broc_std::BrocStr::from("a"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_fail_surrogate_half() {
    assert_evals_to!(
        indoc!(
            r#"
                    when Str.fromUtf8 [97, 98, 0xED, 0xA0, 0x80] is
                        Err (BadUtf8 EncodesSurrogateHalf byteIndex) ->
                            if byteIndex == 2 then
                                "a"
                            else
                                "b"
                        _ -> ""
                "#
        ),
        broc_std::BrocStr::from("a"),
        broc_std::BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_equality() {
    assert_evals_to!(r#""a" == "a""#, true, bool);
    assert_evals_to!(
        r#""loremipsumdolarsitamet" == "loremipsumdolarsitamet""#,
        true,
        bool
    );
    assert_evals_to!(r#""a" != "b""#, true, bool);
    assert_evals_to!(r#""a" == "b""#, false, bool);
}

#[test]
fn str_clone() {
    use broc_std::BrocStr;
    let long = BrocStr::from("loremipsumdolarsitamet");
    let short = BrocStr::from("x");
    let empty = BrocStr::from("");

    debug_assert_eq!(long.clone(), long);
    debug_assert_eq!(short.clone(), short);
    debug_assert_eq!(empty.clone(), empty);
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn nested_recursive_literal() {
    assert_evals_to!(
        indoc!(
            r#"
                Expr : [Add Expr Expr, Val I64, Var I64]

                expr : Expr
                expr = Add (Add (Val 3) (Val 1)) (Add (Val 1) (Var 1))

                printExpr : Expr -> Str
                printExpr = \e ->
                    when e is
                        Add a b ->
                            "Add ("
                                |> Str.concat (printExpr a)
                                |> Str.concat ") ("
                                |> Str.concat (printExpr b)
                                |> Str.concat ")"
                        Val v -> "Val " |> Str.concat (Num.toStr v)
                        Var v -> "Var " |> Str.concat (Num.toStr v)

                printExpr expr
                "#
        ),
        BrocStr::from("Add (Add (Val 3) (Val 1)) (Add (Val 1) (Var 1))"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_join_comma_small() {
    assert_evals_to!(
        r#"Str.joinWith ["1", "2"] ", " "#,
        BrocStr::from("1, 2"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_join_comma_big() {
    assert_evals_to!(
        r#"Str.joinWith ["10000000", "2000000", "30000000"] ", " "#,
        BrocStr::from("10000000, 2000000, 30000000"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_join_comma_single() {
    assert_evals_to!(r#"Str.joinWith ["1"] ", " "#, BrocStr::from("1"), BrocStr);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_to_utf8() {
    assert_evals_to!(
        r#"Str.toUtf8 "hello""#,
        BrocList::from_slice(&[104, 101, 108, 108, 111]),
        BrocList<u8>
    );
    assert_evals_to!(
        r#"Str.toUtf8 "this is a long string""#,
        BrocList::from_slice(&[
            116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 108, 111, 110, 103, 32, 115, 116, 114,
            105, 110, 103
        ]),
        BrocList<u8>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_range() {
    assert_evals_to!(
        indoc!(
            r#"
            bytes = Str.toUtf8 "hello"
            when Str.fromUtf8Range bytes { count: 5,  start: 0 }  is
                   Ok utf8String -> utf8String
                   _ -> ""
            "#
        ),
        BrocStr::from("hello"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_range_slice() {
    assert_evals_to!(
        indoc!(
            r#"
            bytes = Str.toUtf8 "hello"
            when Str.fromUtf8Range bytes { count: 4,  start: 1 }  is
                   Ok utf8String -> utf8String
                   _ -> ""
            "#
        ),
        BrocStr::from("ello"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_range_slice_not_end() {
    assert_evals_to!(
        indoc!(
            r#"
            bytes = Str.toUtf8 "hello"
            when Str.fromUtf8Range bytes { count: 3,  start: 1 }  is
                   Ok utf8String -> utf8String
                   _ -> ""
            "#
        ),
        BrocStr::from("ell"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_range_order_does_not_matter() {
    assert_evals_to!(
        indoc!(
            r#"
            bytes = Str.toUtf8 "hello"
            when Str.fromUtf8Range bytes { start: 1,  count: 3 }  is
                   Ok utf8String -> utf8String
                   _ -> ""
            "#
        ),
        BrocStr::from("ell"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_range_out_of_bounds_start_value() {
    assert_evals_to!(
        indoc!(
            r#"
            bytes = Str.toUtf8 "hello"
            when Str.fromUtf8Range bytes { start: 7,  count: 3 }  is
                   Ok _ -> ""
                   Err (BadUtf8 _ _) -> ""
                   Err OutOfBounds -> "out of bounds"
            "#
        ),
        BrocStr::from("out of bounds"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_range_count_too_high() {
    assert_evals_to!(
        indoc!(
            r#"
            bytes = Str.toUtf8 "hello"
            when Str.fromUtf8Range bytes { start: 0,  count: 6 }  is
                   Ok _ -> ""
                   Err (BadUtf8 _ _) -> ""
                   Err OutOfBounds -> "out of bounds"
            "#
        ),
        BrocStr::from("out of bounds"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_from_utf8_range_count_too_high_for_start() {
    assert_evals_to!(
        indoc!(
            r#"
            bytes = Str.toUtf8 "hello"
            when Str.fromUtf8Range bytes { start: 4,  count: 3 }  is
                   Ok _ -> ""
                   Err (BadUtf8 _ _) -> ""
                   Err OutOfBounds -> "out of bounds"
            "#
        ),
        BrocStr::from("out of bounds"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_repeat_small_stays_small() {
    assert_evals_to!(
        indoc!(r#"Str.repeat "Broc" 3"#),
        BrocStr::from("BrocBrocBroc"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_repeat_small_becomes_big() {
    assert_evals_to!(
        indoc!(r#"Str.repeat "less than 23 characters" 2"#),
        BrocStr::from("less than 23 charactersless than 23 characters"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_repeat_big() {
    assert_evals_to!(
        indoc!(r#"Str.repeat "more than 23 characters now" 2"#),
        BrocStr::from("more than 23 characters nowmore than 23 characters now"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_repeat_empty_string() {
    let a = indoc!(r#"Str.repeat "" 3"#);
    assert_evals_to!(a, BrocStr::from(""), BrocStr);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_repeat_zero_times() {
    assert_evals_to!(indoc!(r#"Str.repeat "Broc" 0"#), BrocStr::from(""), BrocStr);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_empty_string() {
    assert_evals_to!(indoc!(r#"Str.trim """#), BrocStr::from(""), BrocStr);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_null_byte() {
    assert_evals_to!(
        indoc!(r#"Str.trim (Str.reserve "\u(0000)" 40)"#),
        BrocStr::from("\0"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_small_blank_string() {
    assert_evals_to!(indoc!(r#"Str.trim " ""#), BrocStr::from(""), BrocStr);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_small_to_small() {
    assert_evals_to!(
        indoc!(r#"Str.trim "  hello world  ""#),
        BrocStr::from("hello world"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_large_to_large_unique() {
    assert_evals_to!(
        indoc!(r#"Str.trim (Str.concat "  " "hello world from a large string ")"#),
        BrocStr::from("hello world from a large string"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_large_to_small_unique() {
    assert_evals_to!(
        indoc!(r#"Str.trim (Str.concat "  " "hello world        ")"#),
        BrocStr::from("hello world"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_trim_large_to_large_shared() {
    assert_evals_to!(
        indoc!(
            r#"
               original : Str
               original = " hello world world "

               { trimmed: Str.trim original, original: original }
               "#
        ),
        (
            BrocStr::from(" hello world world "),
            BrocStr::from("hello world world"),
        ),
        (BrocStr, BrocStr)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_trim_large_to_small_shared() {
    assert_evals_to!(
        indoc!(
            r#"
               original : Str
               original = " hello world             "

               { trimmed: Str.trim original, original: original }
               "#
        ),
        (
            BrocStr::from(" hello world             "),
            BrocStr::from("hello world"),
        ),
        (BrocStr, BrocStr)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_trim_small_to_small_shared() {
    assert_evals_to!(
        indoc!(
            r#"
               original : Str
               original = " hello world "

               { trimmed: Str.trim original, original: original }
               "#
        ),
        (BrocStr::from(" hello world "), BrocStr::from("hello world"),),
        (BrocStr, BrocStr)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_left_small_blank_string() {
    assert_evals_to!(indoc!(r#"Str.trimLeft " ""#), BrocStr::from(""), BrocStr);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_left_small_to_small() {
    assert_evals_to!(
        indoc!(r#"Str.trimLeft "  hello world  ""#),
        BrocStr::from("hello world  "),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_left_large_to_large_unique() {
    assert_evals_to!(
        indoc!(r#"Str.trimLeft (Str.concat "    " "hello world from a large string ")"#),
        BrocStr::from("hello world from a large string "),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_left_large_to_small_unique() {
    assert_evals_to!(
        indoc!(r#"Str.trimLeft (Str.concat "  " "hello world        ")"#),
        BrocStr::from("hello world        "),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_trim_left_large_to_large_shared() {
    assert_evals_to!(
        indoc!(
            r#"
               original : Str
               original = " hello world world "

               { trimmed: Str.trimLeft original, original: original }
               "#
        ),
        (
            BrocStr::from(" hello world world "),
            BrocStr::from("hello world world "),
        ),
        (BrocStr, BrocStr)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_trim_left_large_to_small_shared() {
    assert_evals_to!(
        indoc!(
            r#"
               original : Str
               original = " hello world             "

               { trimmed: Str.trimLeft original, original: original }
               "#
        ),
        (
            BrocStr::from(" hello world             "),
            BrocStr::from("hello world             "),
        ),
        (BrocStr, BrocStr)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_trim_left_small_to_small_shared() {
    assert_evals_to!(
        indoc!(
            r#"
               original : Str
               original = " hello world "

               { trimmed: Str.trimLeft original, original: original }
               "#
        ),
        (BrocStr::from(" hello world "), BrocStr::from("hello world "),),
        (BrocStr, BrocStr)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_right_small_blank_string() {
    assert_evals_to!(indoc!(r#"Str.trimRight " ""#), BrocStr::from(""), BrocStr);
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_right_small_to_small() {
    assert_evals_to!(
        indoc!(r#"Str.trimRight "  hello world  ""#),
        BrocStr::from("  hello world"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_right_large_to_large_unique() {
    assert_evals_to!(
        indoc!(r#"Str.trimRight (Str.concat " hello world from a large string" "    ")"#),
        BrocStr::from(" hello world from a large string"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_trim_right_large_to_small_unique() {
    assert_evals_to!(
        indoc!(r#"Str.trimRight (Str.concat "        hello world" "  ")"#),
        BrocStr::from("        hello world"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_trim_right_large_to_large_shared() {
    assert_evals_to!(
        indoc!(
            r#"
               original : Str
               original = " hello world world "

               { trimmed: Str.trimRight original, original: original }
               "#
        ),
        (
            BrocStr::from(" hello world world "),
            BrocStr::from(" hello world world"),
        ),
        (BrocStr, BrocStr)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_trim_right_large_to_small_shared() {
    assert_evals_to!(
        indoc!(
            r#"
               original : Str
               original = "             hello world "

               { trimmed: Str.trimRight original, original: original }
               "#
        ),
        (
            BrocStr::from("             hello world "),
            BrocStr::from("             hello world"),
        ),
        (BrocStr, BrocStr)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_trim_right_small_to_small_shared() {
    assert_evals_to!(
        indoc!(
            r#"
               original : Str
               original = " hello world "

               { trimmed: Str.trimRight original, original: original }
               "#
        ),
        (BrocStr::from(" hello world "), BrocStr::from(" hello world"),),
        (BrocStr, BrocStr)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_to_nat() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toNat "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<usize, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_i128() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toI128 "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<i128, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_u128() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toU128 "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<u128, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_to_i64() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toI64 "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<i64, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_to_u64() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toU64 "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<u64, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_i32() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toI32 "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<i32, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_u32() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toU32 "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<u32, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_i16() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toI16 "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<i16, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_u16() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toU16 "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<u16, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_i8() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toI8 "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<i8, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_u8() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toU8 "1"
            "#
        ),
        BrocResult::ok(1),
        BrocResult<u8, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_f64() {
    assert_evals_to!(
        indoc!(
            r#"
            when Str.toF64 "1.0" is
                Ok n -> n
                Err _ -> 0

            "#
        ),
        1.0,
        f64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_f32() {
    assert_evals_to!(
        indoc!(
            r#"
            when Str.toF32 "1.0" is
                Ok n -> n
                Err _ -> 0

            "#
        ),
        1.0,
        f32
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_to_dec() {
    use broc_std::BrocDec;

    assert_evals_to!(
        indoc!(
            r#"
            when Str.toDec "1.0" is
                Ok n -> n
                Err _ -> 0

            "#
        ),
        BrocDec::from_str("1.0").unwrap(),
        BrocDec
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn issue_2811() {
    assert_evals_to!(
        indoc!(
            r#"
            x = Command { tool: "bash" }
            Command c = x
            c.tool
            "#
        ),
        BrocStr::from("bash"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn to_scalar_1_byte() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toScalars "R"
            "#
        ),
        BrocList::from_slice(&[82u32]),
        BrocList<u32>
    );

    assert_evals_to!(
        indoc!(
            r#"
            Str.toScalars "Broc!"
            "#
        ),
        BrocList::from_slice(&[82u32, 111, 99, 33]),
        BrocList<u32>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn to_scalar_2_byte() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toScalars "é"
            "#
        ),
        BrocList::from_slice(&[233u32]),
        BrocList<u32>
    );

    assert_evals_to!(
        indoc!(
            r#"
            Str.toScalars "Cäfés"
            "#
        ),
        BrocList::from_slice(&[67u32, 228, 102, 233, 115]),
        BrocList<u32>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn to_scalar_3_byte() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.toScalars "鹏"
            "#
        ),
        BrocList::from_slice(&[40527u32]),
        BrocList<u32>
    );

    assert_evals_to!(
        indoc!(
            r#"
            Str.toScalars "鹏很有趣"
            "#
        ),
        BrocList::from_slice(&[40527u32, 24456, 26377, 36259]),
        BrocList<u32>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn to_scalar_4_byte() {
    // from https://design215.com/toolbox/utf8-4byte-characters.php
    assert_evals_to!(
        indoc!(
            r#"
            Str.toScalars "𒀀"
            "#
        ),
        BrocList::from_slice(&[73728u32]),
        BrocList<u32>
    );

    assert_evals_to!(
        indoc!(
            r#"
            Str.toScalars "𒀀𒀁"
            "#
        ),
        BrocList::from_slice(&[73728u32, 73729u32]),
        BrocList<u32>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_first_one_char() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.splitFirst "foo/bar/baz" "/"
            "#
        ),
        // the result is a { before, after } record, and because of
        // alphabetic ordering the fields here are flipped
        BrocResult::ok((BrocStr::from("bar/baz"), BrocStr::from("foo"))),
        BrocResult<(BrocStr, BrocStr), ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_first_multiple_chars() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.splitFirst "foo//bar//baz" "//"
            "#
        ),
        BrocResult::ok((BrocStr::from("bar//baz"), BrocStr::from("foo"))),
        BrocResult<(BrocStr, BrocStr), ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_first_entire_input() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.splitFirst "foo" "foo"
            "#
        ),
        BrocResult::ok((BrocStr::from(""), BrocStr::from(""))),
        BrocResult<(BrocStr, BrocStr), ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_first_not_found() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.splitFirst "foo" "bar"
            "#
        ),
        BrocResult::err(()),
        BrocResult<(BrocStr, BrocStr), ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_last_one_char() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.splitLast"foo/bar/baz" "/"
            "#
        ),
        BrocResult::ok((BrocStr::from("baz"), BrocStr::from("foo/bar"))),
        BrocResult<(BrocStr, BrocStr), ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_last_multiple_chars() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.splitLast "foo//bar//baz" "//"
            "#
        ),
        BrocResult::ok((BrocStr::from("baz"), BrocStr::from("foo//bar"))),
        BrocResult<(BrocStr, BrocStr), ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_last_entire_input() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.splitLast "foo" "foo"
            "#
        ),
        BrocResult::ok((BrocStr::from(""), BrocStr::from(""))),
        BrocResult<(BrocStr, BrocStr), ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_last_not_found() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.splitLast "foo" "bar"
            "#
        ),
        BrocResult::err(()),
        BrocResult<(BrocStr, BrocStr), ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_overlapping_substring_1() {
    assert_evals_to!(
        r#"Str.split "aaa" "aa""#,
        BrocList::from_slice(&[BrocStr::from(""), BrocStr::from("a")]),
        BrocList<BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_split_overlapping_substring_2() {
    assert_evals_to!(
        r#"Str.split "aaaa" "aa""#,
        BrocList::from_slice(&[BrocStr::from(""), BrocStr::from(""), BrocStr::from("")]),
        BrocList<BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_walk_utf8() {
    #[cfg(not(feature = "gen-llvm-wasm"))]
    assert_evals_to!(
        // Reverse the bytes
        indoc!(
            r#"
            Str.walkUtf8 "abcd" [] (\list, byte -> List.prepend list byte)
            "#
        ),
        BrocList::from_slice(&[b'd', b'c', b'b', b'a']),
        BrocList<u8>
    );

    #[cfg(feature = "gen-llvm-wasm")]
    assert_evals_to!(
        indoc!(
            r#"
            Str.walkUtf8WithIndex "abcd" [] (\list, byte, index -> List.append list (Pair index byte))
            "#
        ),
        BrocList::from_slice(&[(0, 'a'), (1, 'b'), (2, 'c'), (3, 'd')]),
        BrocList<(u32, char)>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_walk_utf8_with_index() {
    #[cfg(not(feature = "gen-llvm-wasm"))]
    assert_evals_to!(
        indoc!(
            r#"
            Str.walkUtf8WithIndex "abcd" [] (\list, byte, index -> List.append list (Pair index byte))
            "#
        ),
        BrocList::from_slice(&[(0, b'a'), (1, b'b'), (2, b'c'), (3, b'd')]),
        BrocList<(u64, u8)>
    );

    #[cfg(feature = "gen-llvm-wasm")]
    assert_evals_to!(
        indoc!(
            r#"
            Str.walkUtf8WithIndex "abcd" [] (\list, byte, index -> List.append list (Pair index byte))
            "#
        ),
        BrocList::from_slice(&[(0, 'a'), (1, 'b'), (2, 'c'), (3, 'd')]),
        BrocList<(u32, char)>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm"))]
fn str_append_scalar() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.appendScalar "abcd" 'A'
            "#
        ),
        BrocStr::from("abcdA"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-dev"))]
fn str_walk_scalars() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.walkScalars "abcd" [] List.append
            "#
        ),
        BrocList::from_slice(&['a', 'b', 'c', 'd']),
        BrocList<char>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm-wasm"))]
fn llvm_wasm_str_layout() {
    assert_evals_to!(
        indoc!(
            r#"
            "hello"
                |> Str.reserve 42
            "#
        ),
        [0, 5, 1],
        [u32; 3],
        |[_ptr, len, cap]: [u32; 3]| [0, len, if cap >= 42 { 1 } else { 0 }]
    )
}

#[test]
#[cfg(any(feature = "gen-llvm-wasm"))]
fn llvm_wasm_str_layout_small() {
    // exposed an error in using bitcast instead of zextend
    assert_evals_to!(
        indoc!(
            r#"
            "𒀀𒀁"
                |> Str.trim
            "#
        ),
        [-2139057424, -2122280208, -2013265920],
        [i32; 3]
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn when_on_strings() {
    assert_evals_to!(
        indoc!(
            r#"
            when "Deyr fé, deyja frændr" is
                "Deyr fé, deyja frændr" -> 42
                "deyr sjalfr it sama" -> 1
                "en orðstírr deyr aldregi" -> 2
                "hveim er sér góðan getr" -> 3
                _ -> 4
            "#
        ),
        42,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
            when "Deyr fé, deyja frændr" is
                "deyr sjalfr it sama" -> 1
                "en orðstírr deyr aldregi" -> 2
                "hveim er sér góðan getr" -> 3
                _ -> 4
            "#
        ),
        4,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn with_capacity() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.withCapacity 10
            "#
        ),
        BrocStr::from(""),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn with_capacity_concat() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.withCapacity 10 |> Str.concat "Forty-two"
            "#
        ),
        BrocStr::from("Forty-two"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn str_with_prefix() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.withPrefix "world!" "Hello "
            "#
        ),
        BrocStr::from("Hello world!"),
        BrocStr
    );

    assert_evals_to!(
        indoc!(
            r#"
            "two" |> Str.withPrefix "Forty "
            "#
        ),
        BrocStr::from("Forty two"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn destructure_pattern_assigned_from_thunk_opaque() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [main] to "./platform"

            MyCustomType := Str
            myMsg = @MyCustomType "Hello"

            main =
                @MyCustomType msg = myMsg

                msg
            "#
        ),
        BrocStr::from("Hello"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn destructure_pattern_assigned_from_thunk_tag() {
    assert_evals_to!(
        indoc!(
            r#"
            app "test" provides [main] to "./platform"

            myMsg = A "hello " "world"

            main =
                A m1 m2 = myMsg

                Str.concat m1 m2
            "#
        ),
        BrocStr::from("hello world"),
        BrocStr
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn release_excess_capacity() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.reserve "" 50
            |> Str.releaseExcessCapacity
            "#
        ),
        (BrocStr::empty().capacity(), BrocStr::empty()),
        BrocStr,
        |value: BrocStr| (value.capacity(), value)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn release_excess_capacity_with_len() {
    assert_evals_to!(
        indoc!(
            r#"
            "123456789012345678901234567890"
            |> Str.reserve 50
            |> Str.releaseExcessCapacity
            "#
        ),
        (30, "123456789012345678901234567890".into()),
        BrocStr,
        |value: BrocStr| (value.capacity(), value)
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn release_excess_capacity_empty() {
    assert_evals_to!(
        indoc!(
            r#"
            Str.releaseExcessCapacity ""
            "#
        ),
        (BrocStr::empty().capacity(), BrocStr::empty()),
        BrocStr,
        |value: BrocStr| (value.capacity(), value)
    );
}
