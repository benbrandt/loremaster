#[allow(dead_code)]
pub mod loremaster {
    #[allow(dead_code)]
    pub mod cultures {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            /// Available Heroic Cultures for Player Characters.
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum HeroicCulture {
                /// All Bardings speak Dalish, a language that can be described as a very old form of the
                /// Common Speech. As far as their names are concerned, they are usually composed of one or two
                /// elements (for example, Dag — Day, or Lif-stan — Life Stone). Like most Northmen, Bardings
                /// often name their children after a renowned ancestor or relative, or choose a name beginning
                /// with the same sound or sharing one element with that of the father (whose name is often
                /// given with their first name when introduced formally — for example, Lifstan, son of
                /// Leiknir, or Ingrith, daughter of Ingolf).
                Bardings,
                /// All Dwarves speak the Common Tongue, but preserve a knowledge of a secret Dwarvish
                /// language. They receive a true name at birth that they do not reveal to members of other
                /// folks, and adopt another name in the tradition of their neighbours. This custom has been in
                /// use for so long that a number of names have become traditionally associated with Dwarves,
                /// and are used almost exclusively by them. Dwarves of renown are sometimes given an honorific
                /// title, celebrating an exceptional deed or distinctive quality (for example, Thorin
                /// Oakenshield or Dáin Ironfoot).
                DwarvesOfDurinsFolk,
                /// In addition to the Common Speech, all Elves speak their own, fair tongue — the Sindarin
                /// speech. For the most part, the Elves of Lindon bear names fashioned in that language.
                ElvesOfLindon,
                /// Hobbits speak only the Common Speech, preserving the use of a few words and names of their
                /// own forgotten tongue. Names are composed of a first name and a family name. First names for
                /// men are usually simple and short, with women being often given names of flowers or precious
                /// stones, but among the older families a custom survives of giving more heroic and
                /// high-sounding names, whose origin can be traced back to a time before the Shire.
                HobbitsOfTheShire,
                /// The Men of Bree have forgotten their ancient, native speech, and speak the Common Tongue,
                /// albeit slightly altered in a local dialect. They use names that to foreign ears sound
                /// similar to those used by Hobbits in the Shire (Hobbits beg to differ, of course).
                MenOfBree,
                /// The native language of the Dúnedain is the Westron, or Common Speech. Some still learn the
                /// Sindarin Elven-tongue, as it is handed down from generation to generation. They retain an
                /// ancient tradition of naming their children using that fair speech.
                RangersOfTheNorth,
            }
            impl ::core::fmt::Debug for HeroicCulture {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        HeroicCulture::Bardings => {
                            f.debug_tuple("HeroicCulture::Bardings").finish()
                        }
                        HeroicCulture::DwarvesOfDurinsFolk => {
                            f.debug_tuple("HeroicCulture::DwarvesOfDurinsFolk").finish()
                        }
                        HeroicCulture::ElvesOfLindon => {
                            f.debug_tuple("HeroicCulture::ElvesOfLindon").finish()
                        }
                        HeroicCulture::HobbitsOfTheShire => {
                            f.debug_tuple("HeroicCulture::HobbitsOfTheShire").finish()
                        }
                        HeroicCulture::MenOfBree => {
                            f.debug_tuple("HeroicCulture::MenOfBree").finish()
                        }
                        HeroicCulture::RangersOfTheNorth => {
                            f.debug_tuple("HeroicCulture::RangersOfTheNorth").finish()
                        }
                    }
                }
            }
            impl HeroicCulture {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> HeroicCulture {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => HeroicCulture::Bardings,
                        1 => HeroicCulture::DwarvesOfDurinsFolk,
                        2 => HeroicCulture::ElvesOfLindon,
                        3 => HeroicCulture::HobbitsOfTheShire,
                        4 => HeroicCulture::MenOfBree,
                        5 => HeroicCulture::RangersOfTheNorth,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod loremaster {
        #[allow(dead_code)]
        pub mod cultures {
            #[allow(dead_code, clippy::all)]
            pub mod generate {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type HeroicCulture =
                    super::super::super::super::loremaster::cultures::types::HeroicCulture;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_generate_culture_cabi<T: Guest>() -> i32 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::generate_culture();
                    result0.clone() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_generate_name_cabi<T: Guest>(arg0: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::generate_name(
                        super::super::super::super::loremaster::cultures::types::HeroicCulture::_lift(
                            arg0 as u8,
                        ),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_generate_name<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    /// Generate a random Heroic Culture.
                    fn generate_culture() -> HeroicCulture;
                    /// Generate a random name for a given Heroic Culture.
                    fn generate_name(culture: HeroicCulture) -> _rt::String;
                }
                #[doc(hidden)]
                macro_rules! __export_loremaster_cultures_generate_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "loremaster:cultures/generate#generate-culture"] unsafe extern
                        "C" fn export_generate_culture() -> i32 { $($path_to_types)*::
                        _export_generate_culture_cabi::<$ty > () } #[export_name =
                        "loremaster:cultures/generate#generate-name"] unsafe extern "C"
                        fn export_generate_name(arg0 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_generate_name_cabi::<$ty > (arg0) }
                        #[export_name =
                        "cabi_post_loremaster:cultures/generate#generate-name"] unsafe
                        extern "C" fn _post_return_generate_name(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_generate_name::<$ty > (arg0) }
                        };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_loremaster_cultures_generate_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 8]);
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::alloc;
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_cultures_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::loremaster::cultures::generate::__export_loremaster_cultures_generate_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::loremaster::cultures::generate);
    };
}
#[doc(inline)]
pub(crate) use __export_cultures_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:loremaster:cultures:cultures:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 462] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xcf\x02\x01A\x02\x01\
A\x05\x01B\x02\x01m\x06\x08bardings\x16dwarves-of-durins-folk\x0felves-of-lindon\
\x14hobbits-of-the-shire\x0bmen-of-bree\x14rangers-of-the-north\x04\0\x0eheroic-\
culture\x03\0\0\x03\x01\x19loremaster:cultures/types\x05\0\x02\x03\0\0\x0eheroic\
-culture\x01B\x06\x02\x03\x02\x01\x01\x04\0\x0eheroic-culture\x03\0\0\x01@\0\0\x01\
\x04\0\x10generate-culture\x01\x02\x01@\x01\x07culture\x01\0s\x04\0\x0dgenerate-\
name\x01\x03\x04\x01\x1cloremaster:cultures/generate\x05\x02\x04\x01\x1cloremast\
er:cultures/cultures\x04\0\x0b\x0e\x01\0\x08cultures\x03\0\0\0G\x09producers\x01\
\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
