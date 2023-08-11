use unocss_classes::{to_uno, uno};

macro_rules! input {
    ($($t:tt)*) => {
        (uno![$($t)*], to_uno![$($t)*])
    };
}

macro_rules! test_cases {
    ($($t:tt)*) => {
        let cases = vec![$($t)*];

        for ((uno_result, to_uno_result), expected_result) in cases {
            assert_eq!(uno_result, expected_result);
            assert_eq!(to_uno_result, expected_result);
        }
    };
}

#[test]
fn mimics_original_transformer_behavior() {
    test_cases![
        (
            input!["hover:(bg-gray-400 font-medium)", "font-(light mono)"],
            "hover:bg-gray-400 hover:font-medium font-light font-mono",
        ),
        // test cases taken from
        // https://github.com/unocss/unocss/blob/main/test/transformer-variant-group.test.ts
        (
            input!["a1 a2:(b1 b2:(c1 c2-(d1 d2) c3) b3) a3"],
            "a1 a2:b1 a2:b2:c1 a2:b2:c2-d1 a2:b2:c2-d2 a2:b2:c3 a2:b3 a3",
        ),
        (
            input!["bg-white font-light sm:hover:(bg-gray-100 font-medium)"],
            "bg-white font-light sm:hover:bg-gray-100 sm:hover:font-medium",
        ),
        (
            input!["lt-sm:hover:(p-1 p-2)"],
            "lt-sm:hover:p-1 lt-sm:hover:p-2",
        ),
        (input!["<sm:hover:(p-1 p-2)"], "<sm:hover:p-1 <sm:hover:p-2"),
        (input!["sm:(p-1 p-2)"], "sm:p-1 sm:p-2"),
        (input!["dark:lg:(p-1 p-2)"], "dark:lg:p-1 dark:lg:p-2"),
        (input!["at-lg:(p-1 p-2)"], "at-lg:p-1 at-lg:p-2"),
        (input!["md:(w-40vw pr-4.5rem)"], "md:w-40vw md:pr-4.5rem"),
        (
            input!["lt-md:(grid grid-cols-[1fr,50%])"],
            "lt-md:grid lt-md:grid-cols-[1fr,50%]",
        ),
        (
            input!["<md:(grid grid-cols-[1fr,50%])"],
            "<md:grid <md:grid-cols-[1fr,50%]",
        ),
        (input!["!hover:(m-2 p-2)"], "!hover:m-2 !hover:p-2"),
        (input!["hover:(\n!m-2\np-2\n)"], "!hover:m-2 hover:p-2"),
        (input!["hover:(\n!m-2 \np-2\n)"], "!hover:m-2 hover:p-2"),
        (
            input!["md:(w-1/2 h-[calc(100%-4rem)])"],
            "md:w-1/2 md:h-[calc(100%-4rem)]",
        ),
        (
            input!["[&]:(w-4 h-4) [&]:(w-4 h-4)"],
            "[&]:w-4 [&]:h-4 [&]:w-4 [&]:h-4",
        ),
        // test cases taken from
        // https://github.com/unocss/unocss/blob/main/test/variant-group.test.ts
        (input![""], ""),
        (input!["a b c"], "a b c"),
        (input!["a:b:c"], "a:b:c"),
        (input!["hello a:(b c) c:(a:b d)"], "hello a:b a:c c:a:b c:d"),
        (input!["b:c:d:(!a z)"], "!b:c:d:a b:c:d:z"),
        (input!["a-(b c) c-(a:b d)"], "a-b a-c c-a:b c-d"),
        (input!["a-(~ b c)"], "a a-b a-c"),
        (input!["a-(b c-(d e f))"], "a-b a-c-d a-c-e a-c-f"),
        (input!["a-( ~ b c )"], "a a-b a-c"),
        (
            input!["b:[&:not(c)]:d:(!a z)"],
            "!b:[&:not(c)]:d:a b:[&:not(c)]:d:z",
        ),
        (input!["[&]:(a-b c-d)"], "[&]:a-b [&]:c-d"),
        (input!["  a:(b:(c-d d-c)) "], "a:b:c-d a:b:d-c"),
        (input!["@a:(c-d d-c)"], "@a:c-d @a:d-c"),
        (input!["!@a:(c-d d-c)"], "!@a:c-d !@a:d-c"),
        (input!["a:(b?c d)"], "a:b?c a:d"),
    ];
}

#[test]
fn transforms_inconsistent_whitespaces() {
    test_cases![
        (
            input![" \n \t text-(red\nlg:(sm blue)) \tm-(t1\n\t r-2)  \n "],
            "text-red text-lg:sm text-lg:blue m-t1 m-r-2",
        ),
        (
            input![
                r"p-(x4 y5)
text-red
    text-lg
            sm:fw300
                m-(t1 r-2)"
            ],
            "p-x4 p-y5 text-red text-lg sm:fw300 m-t1 m-r-2",
        ),
    ];
}

#[test]
fn mimics_original_classes_behavior() {
    // test cases taken from
    // https://github.com/sparten11740/classes/blob/main/src/core.rs
    const DISABLED: bool = true;
    test_cases![
        (
            input!["button".to_string(), "button--disabled"],
            "button button--disabled",
        ),
        (
            input![
                Some("button--active"),
                None::<String>,
                Some("button--disabled".to_string())
            ],
            "button--active button--disabled",
        ),
        (
            input![
                "concatenated".to_string() + "-class",
                Some("batman").map(|_| "bruce-wayne")
            ],
            "concatenated-class bruce-wayne",
        ),
        (
            input!["button" => true, "button--disabled" => DISABLED, "button--active" => false, "all-the-buttons" => 42 > 3 ],
            "button button--disabled all-the-buttons",
        ),
        (
            input!["button" => true, Some("button--disabled"), None::<String>, "button--primary"],
            "button button--disabled button--primary",
        ),
        (
            input!["button", "", "button--active"],
            "button button--active",
        ),
        (
            input!["button", "".to_string(), "button--active"],
            "button button--active",
        ),
        (
            input!["button", Some(""), "button--active"],
            "button button--active",
        ),
    ];
}

#[test]
fn mixed_scenario() {
    const BOOL: bool = true;
    const USIZE: usize = 123;
    let some_string: Option<String> = Some("text-black".to_string());

    test_cases![
        (
            input!["text-sm".to_string(), "text-(red nowrap)" => BOOL],
            "text-sm text-red text-nowrap",
        ),
        (
            input!["text-(red nowrap)" => USIZE == 123, some_string.clone()],
            "text-red text-nowrap text-black",
        ),
        (
            input![
                "hover:(bg-gray-400 font-medium)",
                "",
                None::<String>,
                "font-(light mono)",
                ""
            ],
            "hover:bg-gray-400 hover:font-medium font-light font-mono",
        ),
    ];
}
