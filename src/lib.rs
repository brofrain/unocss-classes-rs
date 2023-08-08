/// Builds a single [String] from provided string-like arguments and transforms Variant Groups.
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
        unocss_classes::exports::__uno!($($t)*)
    };
}

pub mod exports {
    pub use classes;
    pub use macros::uno as __uno;
}
