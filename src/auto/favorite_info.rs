// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT


glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FavoriteInfo(Boxed<ffi::XAppFavoriteInfo>);

    match fn {
        copy => |ptr| ffi::xapp_favorite_info_copy(ptr),
        free => |ptr| ffi::xapp_favorite_info_free(ptr),
        type_ => || ffi::xapp_favorite_info_get_type(),
    }
}
