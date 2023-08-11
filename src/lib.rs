/// Builds a single [String] from provided string-like arguments and transforms
/// Variant Groups in string literals. The transformation is executed at compile time,
/// so dynamic values are not supported. In such scenario, use [to_uno!] instead.
///
/// # Example
///
/// ```
/// use unocss_classes::uno;
///
/// assert_eq!(uno!["text-red"], "text-red");
///
/// assert_eq!(
///     uno!["text-(blue lg)", "placeholder:(italic text-(red sm))"],
///     "text-blue text-lg placeholder:italic placeholder:text-red placeholder:text-sm"
/// );
///
/// let some = Some("text-red");
/// let none = None::<String>;
/// let truthy = true;
/// let falsy = false;
/// assert_eq!(uno![
///         some,
///         none,
///         "text-green" => truthy,
///         "text-black" => falsy,
///         "",
///         Some("text-white").map(|_| "text-blue"),
///         None::<String>,
///     ],
///     "text-red text-green text-blue"
/// );
///
/// let truthy = true;
/// assert_eq!(uno!["text-(sm center)" => truthy], "text-sm text-center");
/// ```
#[macro_export]
macro_rules! uno {
    ($($t:tt)*) => {
        unocss_classes::exports::__uno_classes!($($t)*)
    };
}

/// Builds a single [String] from provided string-like arguments and transforms
/// Variant Groups in string literals. **The transformation is executed at runtime**,
/// so unlike [uno!] it has a performace penalty, but can handle dynamic text.
/// Use it only when necessary.
///
/// # Example
///
/// ```
/// use unocss_classes::to_uno;
///
/// let mut dynamic_string = String::from("text-(");
/// dynamic_string.push_str("red sm)");
/// assert_eq!(
///     to_uno![
///         dynamic_string,
///         Some("fw500"),
///         "op".to_string() + "-50"
///     ],
///     "text-red text-sm fw500 op-50"
/// );
/// ```
#[cfg(feature = "runtime")]
#[macro_export]
macro_rules! to_uno {
    ($($t:tt)*) => {
        unocss_classes::exports::__transform_variant_groups(
            unocss_classes::exports::classes::classes!($($t)*)
        )
    };
}

pub mod exports {
    pub use classes;
    pub use macros::uno_classes as __uno_classes;
    pub use utils::transform_variant_groups as __transform_variant_groups;
}
