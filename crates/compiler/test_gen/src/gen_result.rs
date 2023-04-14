#[cfg(feature = "gen-llvm")]
use crate::helpers::llvm::assert_evals_to;

#[cfg(feature = "gen-dev")]
use crate::helpers::dev::assert_evals_to;

#[cfg(feature = "gen-wasm")]
use crate::helpers::wasm::assert_evals_to;

use indoc::indoc;

#[allow(unused_imports)]
use broc_std::{BrocResult, BrocStr};

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn with_default_ok() {
    assert_evals_to!(
        indoc!(
            r#"
            result : Result I64 {}
            result = Ok 12345

            Result.withDefault result 0
            "#
        ),
        12345,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn with_default_err() {
    assert_evals_to!(
        indoc!(
            r#"
            result : Result I64 {}
            result = Err {}

            Result.withDefault result 0
            "#
        ),
        0,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn result_map() {
    assert_evals_to!(
        indoc!(
            r#"
            result : Result I64 {}
            result = Ok 2

            result
                |> Result.map (\x -> x + 1)
                |> Result.withDefault 0
            "#
        ),
        3,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
            result : Result I64 {}
            result = Err {}

            result
                |> Result.map (\x -> x + 1)
                |> Result.withDefault 0
            "#
        ),
        0,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn result_map_err() {
    assert_evals_to!(
        indoc!(
            r#"
            result : Result {} I64
            result = Err 2

            when Result.mapErr result (\x -> x + 1) is
                Err n -> n
                Ok _ -> 0
            "#
        ),
        3,
        i64
    );

    assert_evals_to!(
        indoc!(
            r#"
            result : Result {} I64
            result = Ok {}

            when Result.mapErr result (\x -> x + 1) is
                Err n -> n
                Ok _ -> 0
            "#
        ),
        0,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn err_type_var() {
    assert_evals_to!(
        indoc!(
            r#"
            Result.map (Ok 3) (\x -> x + 1)
                |> Result.withDefault -1
            "#
        ),
        4,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn err_type_var_annotation() {
    assert_evals_to!(
        indoc!(
            r#"
            ok : Result I64 *
            ok = Ok 3

            Result.map ok (\x -> x + 1)
                |> Result.withDefault -1
            "#
        ),
        4,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn err_empty_tag_union() {
    assert_evals_to!(
        indoc!(
            r#"
            ok : Result I64 []
            ok = Ok 3

            Result.map ok (\x -> x + 1)
                |> Result.withDefault -1
            "#
        ),
        4,
        i64
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn is_ok() {
    assert_evals_to!(
        indoc!(
            r#"
            result : Result I64 {}
            result = Ok 2

            Result.isOk result
            "#
        ),
        true,
        bool
    );

    assert_evals_to!(
        indoc!(
            r#"
            result : Result I64 {}
            result = Err {}

            Result.isOk result
            "#
        ),
        false,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn is_err() {
    assert_evals_to!(
        indoc!(
            r#"
            result : Result I64 {}
            result = Ok 2

            Result.isErr result
            "#
        ),
        false,
        bool
    );

    assert_evals_to!(
        indoc!(
            r#"
            result : Result I64 {}
            result = Err {}

            Result.isErr result
            "#
        ),
        true,
        bool
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn broc_result_ok_i64() {
    assert_evals_to!(
        indoc!(
            r#"
            result : Result I64 {}
            result = Ok 42

            result
            "#
        ),
        BrocResult::ok(42),
        BrocResult<i64, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn broc_result_ok_f64() {
    // NOTE: the dev backend does not currently use float registers when returning a more
    // complex type, but the rust side does expect it to. Hence this test fails with gen-dev

    assert_evals_to!(
        indoc!(
            r#"
            result : Result F64 {}
            result = Ok 42.0

            result
            "#
        ),
        BrocResult::ok(42.0),
        BrocResult<f64, ()>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm", feature = "gen-dev"))]
fn broc_result_err() {
    assert_evals_to!(
        indoc!(
            r#"
            result : Result I64 Str
            result = Err "foo"

            result
            "#
        ),
        BrocResult::err(BrocStr::from("foo")),
        BrocResult<i64, BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn issue_2583_specialize_errors_behind_unified_branches() {
    assert_evals_to!(
        r#"
        if Bool.true then List.first [15] else Str.toI64 ""
        "#,
        BrocResult::ok(15i64),
        BrocResult<i64, bool>
    )
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn broc_result_after_on_ok() {
    assert_evals_to!(indoc!(
        r#"
            input : Result I64 Str
            input = Ok 1

            Result.try input \num ->
                if num < 0 then Err "negative!" else Ok -num
            "#),
        BrocResult::ok(-1),
        BrocResult<i64, BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn broc_result_after_on_err() {
    assert_evals_to!(indoc!(
        r#"
            input : Result I64 Str
            input = (Err "already a string")

            Result.try input \num ->
                if num < 0 then Err "negative!" else Ok -num
        "#),
        BrocResult::err(BrocStr::from("already a string")),
        BrocResult<i64, BrocStr>
    );
}

#[test]
#[cfg(any(feature = "gen-llvm", feature = "gen-wasm"))]
fn broc_result_after_err() {
    assert_evals_to!(
        indoc!(
            r#"
            result : Result Str I64
            result =
              Result.onErr (Ok "already a string") \num ->
                if num < 0 then Ok "negative!" else Err -num

            result
            "#
        ),
        BrocResult::ok(BrocStr::from("already a string")),
        BrocResult<BrocStr, i64>
    );

    assert_evals_to!(indoc!(
        r#"
            result : Result Str I64
            result =
              Result.onErr (Err 100) \num ->
                if num < 0 then Ok "negative!" else Err -num

            result
            "#),
        BrocResult::err(-100),
        BrocResult<BrocStr, i64>
    );
}
