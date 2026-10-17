pub use paste;

/// Creates a `#[repr(C, packed)]` struct with automatic padding between
/// fields based on specified byte offsets.
///
/// # Examples
///
/// ```
/// use offsetter::offset;
///
/// offset!(
///     struct Player {
///         0x0 health: f32,
///         0x4 max_health: f32,
///         0x8 team_id: u32,
///     }
/// );
/// ```
///
/// With attributes:
/// ```
/// use offsetter::offset;
///
/// offset!(
///     #[derive(Clone, Copy)]
///     struct Entity {
///         0x0  vtable: u64,
///         0x8  id: u32,
///         0x10 position: [f32; 3],
///     }
/// );
/// ```
#[macro_export]
macro_rules! offset {
    (@guard ($current_offset:expr,) -> {$(#[$attr:meta])* $vis:vis struct $name:ident $(<$($lt:lifetime),*>)? $(($offset:expr, $amount:expr, $fvis:vis $id:ident: $ty:ty))*}) => {
        $crate::paste::paste! {
            #[repr(C, packed)]
            $(#[$attr])*
            $vis struct $name $(<$($lt),*>)? {
                $(
                    [<_pad $id>]: [u8; $amount],
                    $fvis $id: $ty
                ),*
            }
        }
    };

    (@guard ($current_offset:expr, $offset:literal $fvis:vis $id:ident: & $lt:lifetime mut $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        $crate::offset!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)*) -> {$($output)* ($offset, $offset - ($current_offset), $fvis $id: & $lt mut $ty)});
    };

    (@guard ($current_offset:expr, $offset:literal $fvis:vis $id:ident: & $lt:lifetime $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        $crate::offset!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)*) -> {$($output)* ($offset, $offset - ($current_offset), $fvis $id: & $lt $ty)});
    };

    (@guard ($current_offset:expr, $offset:literal $fvis:vis $id:ident: & mut $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        $crate::offset!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)*) -> {$($output)* ($offset, $offset - ($current_offset), $fvis $id: & mut $ty)});
    };

    (@guard ($current_offset:expr, $offset:literal $fvis:vis $id:ident: $ty:ty, $($next:tt)*) -> {$($output:tt)*}) => {
        $crate::offset!(@guard ($offset + core::mem::size_of::<$ty>(), $($next)*) -> {$($output)* ($offset, $offset - ($current_offset), $fvis $id: $ty)});
    };

    (@guard ($current_offset:expr, $offset:literal $fvis:vis $id:ident: $ty:ty) -> {$($output:tt)*}) => {
        $crate::offset!(@guard ($offset + core::mem::size_of::<$ty>(),) -> {$($output)* ($offset, $offset - ($current_offset), $fvis $id: $ty)});
    };

    ($(#[$attr:meta])* $vis:vis struct $name:ident $(<$($lt:lifetime),*>)? { $($input:tt)* }) => {
        $crate::offset!(@guard (0, $($input)*) -> {$(#[$attr])* $vis struct $name $(<$($lt),*>)?});
    };
}
