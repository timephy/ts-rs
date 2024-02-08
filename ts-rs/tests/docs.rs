#![allow(dead_code)]

use std::{concat, fs};

use ts_rs::TS;

/* ============================================================================================== */

/// Doc comment.
/// Supports new lines.
///
/// Testing
#[derive(TS)]
#[ts(export_to = "tests-out/docs/")]
struct A {
    name: String,
}

#[derive(TS)]
#[ts(export_to = "tests-out/docs/")]
/// Doc comment.
/// Supports new lines.
///
/// Testing
struct B {
    name: String,
}

#[derive(TS)]
#[ts(export_to = "tests-out/docs/")]
/// Doc comment.
/// Supports new lines.
///
/// Testing
struct C {}

#[derive(TS)]
#[ts(export_to = "tests-out/docs/")]
/// Doc comment.
/// Supports new lines.
///
/// Testing
struct D;

#[derive(TS)]
#[ts(export_to = "tests-out/docs/")]
/// Doc comment.
/// Supports new lines.
///
/// Testing
enum E {}

#[derive(TS)]
#[ts(export_to = "tests-out/docs/")]
/// Doc comment.
/// Supports new lines.
///
/// Testing
enum F {
    VarA,
}

/* ============================================================================================== */

#[test]
fn export_a() {
    A::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type A = {\n",
            "  name: string;\n",
            "}\n"
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type A = { name: string, }"
        )
    };

    let actual_content = fs::read_to_string("tests-out/docs/A.ts").unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_b() {
    B::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type B = {\n",
            "  name: string;\n",
            "}\n"
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type B = { name: string, }"
        )
    };

    let actual_content = fs::read_to_string("tests-out/docs/B.ts").unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_c() {
    C::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type C = Record<string, never>;\n"
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type C = Record<string, never>;"
        )
    };

    let actual_content = fs::read_to_string("tests-out/docs/C.ts").unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_d() {
    D::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type D = null;\n"
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type D = null;"
        )
    };
    let actual_content = fs::read_to_string("tests-out/docs/D.ts").unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_e() {
    E::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type E = never;\n"
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type E = never;"
        )
    };

    let actual_content = fs::read_to_string("tests-out/docs/E.ts").unwrap();

    assert_eq!(actual_content, expected_content);
}

#[test]
fn export_f() {
    F::export().unwrap();

    let expected_content = if cfg!(feature = "format") {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type F = \"VarA\";\n"
        )
    } else {
        concat!(
            "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\n",
            "/**\n",
            " * Doc comment.\n",
            " * Supports new lines.\n",
            " *\n",
            " * Testing\n",
            " */\n",
            "export type F = \"VarA\";"
        )
    };

    let actual_content = fs::read_to_string("tests-out/docs/F.ts").unwrap();

    assert_eq!(actual_content, expected_content);
}
