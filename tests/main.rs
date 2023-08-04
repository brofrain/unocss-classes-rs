use unocss_variant_group_transformer::uno;

#[test]
fn mimics_original_transformer_behavior() {
    let cases = vec![
        // test cases taken from
        // https://github.com/unocss/unocss/blob/main/test/transformer-variant-group.test.ts
        (
            uno!("a1 a2:(b1 b2:(c1 c2-(d1 d2) c3) b3) a3"),
            "a1 a2:b1 a2:b2:c1 a2:b2:c2-d1 a2:b2:c2-d2 a2:b2:c3 a2:b3 a3",
        ),
        (
            uno!("bg-white font-light sm:hover:(bg-gray-100 font-medium)"),
            "bg-white font-light sm:hover:bg-gray-100 sm:hover:font-medium",
        ),
        (
            uno!("lt-sm:hover:(p-1 p-2)"),
            "lt-sm:hover:p-1 lt-sm:hover:p-2",
        ),
        (uno!("<sm:hover:(p-1 p-2)"), "<sm:hover:p-1 <sm:hover:p-2"),
        (uno!("sm:(p-1 p-2)"), "sm:p-1 sm:p-2"),
        (uno!("dark:lg:(p-1 p-2)"), "dark:lg:p-1 dark:lg:p-2"),
        (uno!("at-lg:(p-1 p-2)"), "at-lg:p-1 at-lg:p-2"),
        (uno!("md:(w-40vw pr-4.5rem)"), "md:w-40vw md:pr-4.5rem"),
        (
            uno!("lt-md:(grid grid-cols-[1fr,50%])"),
            "lt-md:grid lt-md:grid-cols-[1fr,50%]",
        ),
        (
            uno!("<md:(grid grid-cols-[1fr,50%])"),
            "<md:grid <md:grid-cols-[1fr,50%]",
        ),
        (uno!("!hover:(m-2 p-2)"), "!hover:m-2 !hover:p-2"),
        (uno!("hover:(\n!m-2\np-2\n)"), "!hover:m-2 hover:p-2"),
        (uno!("hover:(\n!m-2 \np-2\n)"), "!hover:m-2 hover:p-2"),
        (
            uno!("md:(w-1/2 h-[calc(100%-4rem)])"),
            uno!("md:w-1/2 md:h-[calc(100%-4rem)]"),
        ),
        (
            uno!("[&]:(w-4 h-4) [&]:(w-4 h-4)"),
            "[&]:w-4 [&]:h-4 [&]:w-4 [&]:h-4",
        ),
        // test cases taken from
        // https://github.com/unocss/unocss/blob/main/test/variant-group.test.ts
        (uno!(""), ""),
        (uno!("a b c"), "a b c"),
        (uno!("a:b:c"), "a:b:c"),
        (uno!("hello a:(b c) c:(a:b d)"), "hello a:b a:c c:a:b c:d"),
        (uno!("b:c:d:(!a z)"), "!b:c:d:a b:c:d:z"),
        (uno!("a-(b c) c-(a:b d)"), "a-b a-c c-a:b c-d"),
        (uno!("a-(~ b c)"), "a a-b a-c"),
        (uno!("a-(b c-(d e f))"), "a-b a-c-d a-c-e a-c-f"),
        (uno!("a-( ~ b c )"), "a a-b a-c"),
        (
            uno!("b:[&:not(c)]:d:(!a z)"),
            "!b:[&:not(c)]:d:a b:[&:not(c)]:d:z",
        ),
        (uno!("[&]:(a-b c-d)"), "[&]:a-b [&]:c-d"),
        (uno!("  a:(b:(c-d d-c)) "), "a:b:c-d a:b:d-c"),
        (uno!("@a:(c-d d-c)"), "@a:c-d @a:d-c"),
        (uno!("!@a:(c-d d-c)"), "!@a:c-d !@a:d-c"),
        (uno!("a:(b?c d)"), "a:b?c a:d"),
    ];

    for (result, expected_result) in cases {
        assert_eq!(result, expected_result);
    }
}

#[test]
fn transforms_inconsistent_whitespaces() {
    const CASES: [(&str, &str); 2] = [
        (
            uno!(" \n \t text-(red\nlg:(sm blue)) \tm-(t1\n\t r-2)  \n "),
            "text-red text-lg:sm text-lg:blue m-t1 m-r-2",
        ),
        (
            uno!(
                r"p-(x4 y5)
text-red
    text-lg
            sm:fw300
                m-(t1 r-2)"
            ),
            "p-x4 p-y5 text-red text-lg sm:fw300 m-t1 m-r-2",
        ),
    ];

    for (result, expected_result) in CASES {
        assert_eq!(result, expected_result);
    }
}