#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate indoc;

extern crate dircpy;
extern crate broc_collections;

mod helpers;

#[cfg(test)]
mod glue_cli_run {
    use crate::helpers::fixtures_dir;
    use cli_utils::helpers::{run_glue, run_broc, Out};
    use std::fs;
    use std::path::Path;

    /// This macro does two things.
    ///
    /// First, it generates and runs a separate test for each of the given
    /// expected stdout endings. Each of these should test a particular .broc file
    /// in the fixtures/ directory. The fixtures themselves run assertions too, but
    /// the stdout check verifies that we're actually running the code we think we are;
    /// without it, it would be possible that the fixtures are just exiting without running
    /// any assertions, and we would have no way to find out!
    ///
    /// Second, this generates an extra test which (non-recursively) traverses the
    /// fixtures/ directory and verifies that each of the .broc files in there
    /// has had a corresponding test generated in the previous step. This test
    /// will fail if we ever add a new .broc file to fixtures/ and forget to
    /// add a test for it here!
    macro_rules! fixtures {
        ($($test_name:ident:$fixture_dir:expr => $ends_with:expr,)+) => {
            $(
                #[test]
                #[allow(non_snake_case)]
                fn $test_name() {
                    let dir = fixtures_dir($fixture_dir);

                    generate_glue_for(&dir, std::iter::empty());
                    let out = run_app(&dir.join("app.broc"), std::iter::empty());

                    assert!(out.status.success());
                    let ignorable = "🔨 Rebuilding platform...\n";
                    let stderr = out.stderr.replacen(ignorable, "", 1);
                    assert_eq!(stderr, "");
                    assert!(
                        out.stdout.ends_with($ends_with),
                        "Unexpected stdout ending\n\n  expected:\n\n    {}\n\n  but stdout was:\n\n    {}",
                        $ends_with,
                        out.stdout
                    );
                }
            )*

            #[test]
            #[ignore]
            fn all_fixtures_have_tests() {
                use broc_collections::VecSet;

                let mut all_fixtures: VecSet<String> = VecSet::default();

                $(
                    all_fixtures.insert($fixture_dir.to_string());
                )*

                check_for_tests(&mut all_fixtures);
            }
        }
    }

    fixtures! {
        basic_record:"basic-record" => "Record was: MyRcd { b: 42, a: 1995 }\n",
        nested_record:"nested-record" => "Record was: Outer { y: \"foo\", z: [1, 2], x: Inner { b: 24.0, a: 5 } }\n",
        enumeration:"enumeration" => "tag_union was: MyEnum::Foo, Bar is: MyEnum::Bar, Baz is: MyEnum::Baz\n",
        single_tag_union:"single-tag-union" => indoc!(r#"
            tag_union was: SingleTagUnion::OneTag
        "#),
        union_with_padding:"union-with-padding" => indoc!(r#"
            tag_union was: NonRecursive::Foo("This is a test")
            `Foo "small str"` is: NonRecursive::Foo("small str")
            `Foo "A long enough string to not be small"` is: NonRecursive::Foo("A long enough string to not be small")
            `Bar 123` is: NonRecursive::Bar(123)
            `Baz` is: NonRecursive::Baz(())
            `Blah 456` is: NonRecursive::Blah(456)
        "#),
        union_without_padding:"union-without-padding" => indoc!(r#"
            tag_union was: NonRecursive::Foo("This is a test")
            `Foo "small str"` is: NonRecursive::Foo("small str")
            `Bar 123` is: NonRecursive::Bar(123)
            `Baz` is: NonRecursive::Baz(())
            `Blah 456` is: NonRecursive::Blah(456)
        "#),
        nullable_wrapped:"nullable-wrapped" => indoc!(r#"
            tag_union was: StrFingerTree::More("foo", StrFingerTree::More("bar", StrFingerTree::Empty))
            `More "small str" (Single "other str")` is: StrFingerTree::More("small str", StrFingerTree::Single("other str"))
            `More "small str" Empty` is: StrFingerTree::More("small str", StrFingerTree::Empty)
            `Single "small str"` is: StrFingerTree::Single("small str")
            `Empty` is: StrFingerTree::Empty
        "#),
        nullable_unwrapped:"nullable-unwrapped" => indoc!(r#"
            tag_union was: StrConsList::Cons("World!", StrConsList::Cons("Hello ", StrConsList::Nil))
            `Cons "small str" Nil` is: StrConsList::Cons("small str", StrConsList::Nil)
            `Nil` is: StrConsList::Nil
        "#),
        nonnullable_unwrapped:"nonnullable-unwrapped" => indoc!(r#"
            tag_union was: StrRoseTree::Tree("root", [StrRoseTree::Tree("leaf1", []), StrRoseTree::Tree("leaf2", [])])
            Tree "foo" [] is: StrRoseTree::Tree("foo", [])
        "#),
        basic_recursive_union:"basic-recursive-union" => indoc!(r#"
            tag_union was: Expr::Concat(Expr::String("Hello, "), Expr::String("World!"))
            `Concat (String "Hello, ") (String "World!")` is: Expr::Concat(Expr::String("Hello, "), Expr::String("World!"))
            `String "this is a test"` is: Expr::String("this is a test")
        "#),
        advanced_recursive_union:"advanced-recursive-union" => indoc!(r#"
            rbt was: Rbt { default: Job::Job(R1 { command: Command::Command(R2 { tool: Tool::SystemTool(R4 { name: "test", num: 42 }) }), inputFiles: ["foo"] }) }
        "#),
        list_recursive_union:"list-recursive-union" => indoc!(r#"
            rbt was: Rbt { default: Job::Job(R1 { command: Command::Command(R2 { args: [], tool: Tool::SystemTool(R3 { name: "test" }) }), inputFiles: ["foo"], job: [] }) }
        "#),
        multiple_modules:"multiple-modules" => indoc!(r#"
            combined was: Combined { s1: DepStr1::S("hello"), s2: DepStr2::R("world") }
        "#),
        arguments:"arguments" => indoc!(r#"
            Answer was: 84
        "#),
        brocresult:"brocresult" => indoc!(r#"
            Answer was: BrocOk(ManuallyDrop { value: "Hello World!" })
            Answer was: BrocErr(ManuallyDrop { value: 42 })
        "#),
        option:"option" => indoc!(r#"
            Answer was: "Hello World!"
            Answer was: discriminant_U1::None
        "#),
        return_function:"return-function" => indoc!(r#"
            Answer was: 43 41
        "#),
    }

    fn check_for_tests(all_fixtures: &mut broc_collections::VecSet<String>) {
        use broc_collections::VecSet;

        let fixtures = fixtures_dir("");
        let entries = std::fs::read_dir(fixtures.as_path()).unwrap_or_else(|err| {
            panic!(
                "Error trying to read {} as a fixtures directory: {}",
                fixtures.to_string_lossy(),
                err
            );
        });

        for entry in entries {
            let entry = entry.unwrap();

            if entry.file_type().unwrap().is_dir() {
                let fixture_dir_name = entry.file_name().into_string().unwrap();

                if !all_fixtures.remove(&fixture_dir_name) {
                    panic!(
                        "The glue fixture directory {} does not have any corresponding tests in test_glue_cli. Please add one, so if it ever stops working, we'll know about it right away!",
                        entry.path().to_string_lossy()
                    );
                }
            }
        }

        assert_eq!(all_fixtures, &mut VecSet::default());
    }

    fn generate_glue_for<'a, I: IntoIterator<Item = &'a str>>(
        platform_dir: &'a Path,
        args: I,
    ) -> Out {
        let platform_module_path = platform_dir.join("platform.broc");
        let glue_dir = platform_dir.join("src").join("test_glue");
        let fixture_templates_dir = platform_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("fixture-templates");

        // Copy the rust template from the templates directory into the fixture dir.
        dircpy::CopyBuilder::new(fixture_templates_dir.join("rust"), platform_dir)
            .overwrite(true) // overwrite any files that were already present
            .run()
            .unwrap();

        // Delete the glue file to make sure we're actually regenerating it!
        if glue_dir.exists() {
            fs::remove_dir_all(&glue_dir)
                .expect("Unable to remove test_glue dir in order to regenerate it in the test");
        }

        let rust_glue_spec = fixture_templates_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("src")
            .join("RustGlue.broc");

        // Generate a fresh test_glue for this platform
        let glue_out = run_glue(
            // converting these all to String avoids lifetime issues
            std::iter::once("glue".to_string()).chain(
                args.into_iter().map(|arg| arg.to_string()).chain([
                    rust_glue_spec.to_str().unwrap().to_string(),
                    glue_dir.to_str().unwrap().to_string(),
                    platform_module_path.to_str().unwrap().to_string(),
                ]),
            ),
        );

        let ignorable = "🔨 Rebuilding platform...\n";
        let stderr = glue_out.stderr.replacen(ignorable, "", 1);
        let is_reporting_runtime = stderr.starts_with("runtime: ") && stderr.ends_with("ms\n");
        if !(stderr.is_empty() || is_reporting_runtime) {
            panic!("`broc glue` command had unexpected stderr: {}", stderr);
        }

        assert!(glue_out.status.success(), "bad status {:?}", glue_out);

        glue_out
    }

    fn run_app<'a, I: IntoIterator<Item = &'a str>>(app_file: &'a Path, args: I) -> Out {
        // Generate test_glue for this platform
        let compile_out = run_broc(
            // converting these all to String avoids lifetime issues
            args.into_iter()
                .map(|arg| arg.to_string())
                .chain([app_file.to_str().unwrap().to_string()]),
            &[],
            &[],
        );

        let ignorable = "🔨 Rebuilding platform...\n";
        let stderr = compile_out.stderr.replacen(ignorable, "", 1);
        let is_reporting_runtime = stderr.starts_with("runtime: ") && stderr.ends_with("ms\n");
        if !(stderr.is_empty() || is_reporting_runtime) {
            panic!("`broc` command had unexpected stderr: {}", stderr);
        }

        assert!(compile_out.status.success(), "bad status {:?}", compile_out);

        compile_out
    }
}
