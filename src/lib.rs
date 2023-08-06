/// Transforms Variant Groups in the given string literals.
///
/// # Example
///
/// ```
/// use unocss_classes::uno;
///
/// assert_eq!(uno!("text-red"), "text-red");
///
/// assert_eq!(uno!("text-(red sm)"), "text-red text-sm");
///
/// assert_eq!(
///     uno!("text-(blue lg)", "placeholder:(italic text-(red sm))"),
///     "text-blue text-lg placeholder:italic placeholder:text-red placeholder:text-sm"
/// );
/// ```
#[macro_export]
macro_rules! uno {
    ($($t:tt)*) => {
        unocss_classes::exports::__uno!($($t)*)
    };
}

pub mod exports {
    pub use classes;
    pub use unocss_classes_macro::uno as __uno;
}
