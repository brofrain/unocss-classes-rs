use unocss_variant_group_transformer::uno;

#[test]
fn mimics_original_transformer_behavior() {
    // test cases taken from
    // https://github.com/unocss/unocss/blob/main/test/transformer-variant-group.test.ts
    const CASES: [(&str, &str); 14] = [
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
    ];

    for (result, expected_result) in CASES {
        assert_eq!(result, expected_result);
    }
}
