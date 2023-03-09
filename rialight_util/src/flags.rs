//! A typesafe bitmask flag generator useful for sets of C-style bitmask flags.
//! It can be used for creating typesafe wrappers around C APIs.
//!
//! The `flags!` macro generates `struct`s that manage a set of flags. The
//! flags should only be defined for integer types, otherwise unexpected type
//! errors may occur at compile time.
//!
//! # Example
//!
//! ```
//! use rialight::util::flags::flags;
//!
//! flags! {
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!         const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
//!     }
//! }
//!
//! let e1 = Flags::A | Flags::C;
//! let e2 = Flags::B | Flags::C;
//! assert_eq!((e1 | e2), Flags::ABC);   // union
//! assert_eq!((e1 & e2), Flags::C);     // intersection
//! assert_eq!((e1 - e2), Flags::A);     // set difference
//! assert_eq!(!e2, Flags::A);           // set complement
//! ```
//!
//! See [`example_generated::Flags`](./example_generated/struct.Flags.html) for documentation of code
//! generated by the above `flags!` expansion.
//!
//! The generated `struct`s can also be extended with type and trait
//! implementations:
//!
//! ```
//! use std::fmt;
//!
//! use rialight::util::flags::flags;
//!
//! flags! {
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!     }
//! }
//!
//! impl Flags {
//!     pub fn clear(&mut self) {
//!         self.bits = 0;  // The `bits` field can be accessed from within the
//!                         // same module where the `flags!` macro was invoked.
//!     }
//! }
//!
//! impl fmt::Display for Flags {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         write!(f, "hi!")
//!     }
//! }
//!
//! {
//!     let mut flags = Flags::A | Flags::B;
//!     flags.clear();
//!     assert!(flags.is_empty());
//!     assert_eq!(format!("{}", flags), "hi!");
//!     assert_eq!(format!("{:?}", Flags::A | Flags::B), "A | B");
//!     assert_eq!(format!("{:?}", Flags::B), "B");
//! }
//! ```
//!
//! # Visibility
//!
//! The generated structs and their associated flag constants are not exported
//! out of the current module by default. A definition can be exported out of
//! the current module by adding `pub` before `struct`:
//!
//! ```
//! mod example {
//!     use rialight::util::flags::flags;
//!
//!     flags! {
//!         pub struct Flags1: u32 {
//!             const A = 0b00000001;
//!         }
//!
//! #       pub
//!         struct Flags2: u32 {
//!             const B = 0b00000010;
//!         }
//!     }
//! }
//!
//! {
//!     let flag1 = example::Flags1::A;
//!     let flag2 = example::Flags2::B; // error: const `B` is private
//! }
//! ```
//!
//! # Attributes
//!
//! Attributes can be attached to the generated `struct`s by placing them
//! before the `struct` keyword.
//!
//! ## Representations
//!
//! It's valid to add a `#[repr(C)]` or `#[repr(transparent)]` attribute to a type
//! generated by `flags!`. In these cases, the type is guaranteed to be a newtype.
//!
//! ```
//! use rialight::util::flags::flags;
//!
//! flags! {
//!     #[repr(transparent)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//! ```
//!
//! # Trait implementations
//!
//! The `Copy`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash`
//! traits are automatically derived for the `struct`s using the `derive` attribute.
//! Additional traits can be derived by providing an explicit `derive`
//! attribute on `struct`.
//!
//! The `Extend` and `FromIterator` traits are implemented for the `struct`s,
//! too: `Extend` adds the union of the instances of the `struct` iterated over,
//! while `FromIterator` calculates the union.
//!
//! The `Binary`, `Debug`, `LowerHex`, `Octal` and `UpperHex` traits are also
//! implemented by displaying the bits value of the internal struct.
//!
//! ## Operators
//!
//! The following operator traits are implemented for the generated `struct`s:
//!
//! - `BitOr` and `BitOrAssign`: union
//! - `BitAnd` and `BitAndAssign`: intersection
//! - `BitXor` and `BitXorAssign`: toggle
//! - `Sub` and `SubAssign`: set difference
//! - `Not`: set complement
//!
//! # Methods
//!
//! The following methods are defined for the generated `struct`s:
//!
//! - `empty`: an empty set of flags
//! - `all`: the set of all defined flags
//! - `bits`: the raw value of the flags currently stored
//! - `from_bits`: convert from underlying bit representation, unless that
//!                representation contains bits that do not correspond to a
//!                defined flag
//! - `from_bits_truncate`: convert from underlying bit representation, dropping
//!                         any bits that do not correspond to defined flags
//! - `from_bits_unchecked`: convert from underlying bit representation, keeping
//!                          all bits (even those not corresponding to defined
//!                          flags)
//! - `is_empty`: `true` if no flags are currently stored
//! - `is_all`: `true` if currently set flags exactly equal all defined flags
//! - `intersects`: `true` if there are flags common to both `self` and `other`
//! - `contains`: `true` if all of the flags in `other` are contained within `self`
//! - `insert`: inserts the specified flags in-place
//! - `remove`: removes the specified flags in-place
//! - `toggle`: the specified flags will be inserted if not present, and removed
//!             if they are.
//! - `set`: inserts or removes the specified flags depending on the passed value
//! - `intersection`: returns a new set of flags, containing only the flags present
//!                   in both `self` and `other` (the argument to the function).
//! - `union`: returns a new set of flags, containing any flags present in
//!            either `self` or `other` (the argument to the function).
//! - `difference`: returns a new set of flags, containing all flags present in
//!                 `self` without any of the flags present in `other` (the
//!                 argument to the function).
//! - `symmetric_difference`: returns a new set of flags, containing all flags
//!                           present in either `self` or `other` (the argument
//!                           to the function), but not both.
//! - `complement`: returns a new set of flags, containing all flags which are
//!                 not set in `self`, but which are allowed for this type.
//!
//! ## Default
//!
//! The `Default` trait is not automatically implemented for the generated structs.
//!
//! If your default value is equal to `0` (which is the same value as calling `empty()`
//! on the generated struct), you can simply derive `Default`:
//!
//! ```
//! use rialight::util::flags::flags;
//!
//! flags! {
//!     // Results in default value with bits: 0
//!     #[derive(Default)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! {
//!     let derived_default: Flags = Default::default();
//!     assert_eq!(derived_default.bits(), 0);
//! }
//! ```
//!
//! If your default value is not equal to `0` you need to implement `Default` yourself:
//!
//! ```
//! use rialight::util::flags::flags;
//!
//! flags! {
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! // explicit `Default` implementation
//! impl Default for Flags {
//!     fn default() -> Flags {
//!         Flags::A | Flags::C
//!     }
//! }
//!
//! {
//!     let implemented_default: Flags = Default::default();
//!     assert_eq!(implemented_default, (Flags::A | Flags::C));
//! }
//! ```
//!
//! # Zero Flags
//!
//! Flags with a value equal to zero will have some strange behavior that one should be aware of.
//!
//! ```
//! use rialight::util::flags::flags;
//!
//! flags! {
//!     struct Flags: u32 {
//!         const NONE = 0b00000000;
//!         const SOME = 0b00000001;
//!     }
//! }
//!
//! {
//!     let empty = Flags::empty();
//!     let none = Flags::NONE;
//!     let some = Flags::SOME;
//!
//!     // Zero flags are treated as always present
//!     assert!(empty.contains(Flags::NONE));
//!     assert!(none.contains(Flags::NONE));
//!     assert!(some.contains(Flags::NONE));
//!
//!     // Zero flags will be ignored when testing for emptiness
//!     assert!(none.is_empty());
//! }
//! ```
//!
//! Users should generally avoid defining a flag with a value of zero.

pub use bitflags::bitflags as flags;