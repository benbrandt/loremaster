// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod loremaster {
    #[allow(dead_code)]
    pub mod characters {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type HeroicCulture =
                super::super::super::loremaster::cultures::types::HeroicCulture;
            /// Contains the information necessary to fill out a character sheet.
            #[derive(Clone, serde::Serialize)]
            pub struct Character {
                /// The culture of the character.
                pub heroic_culture: HeroicCulture,
                /// The character's name.
                pub name: _rt::String,
            }
            impl ::core::fmt::Debug for Character {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Character")
                        .field("heroic-culture", &self.heroic_culture)
                        .field("name", &self.name)
                        .finish()
                }
            }
        }

        #[allow(dead_code, clippy::all)]
        pub mod generate {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Character = super::super::super::loremaster::characters::types::Character;
            #[allow(unused_unsafe, clippy::all)]
            /// Generate a random character sheet.
            pub fn generate_character() -> Character {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "loremaster:characters/generate")]
                    extern "C" {
                        #[link_name = "generate-character"]
                        fn wit_import(_: *mut u8);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                    let l3 = *ptr0.add(8).cast::<usize>();
                    let len4 = l3;
                    let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                    super::super::super::loremaster::characters::types::Character {
                        heroic_culture:
                            super::super::super::loremaster::cultures::types::HeroicCulture::_lift(
                                l1 as u8,
                            ),
                        name: _rt::string_lift(bytes4),
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod cultures {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            /// Available Heroic Cultures for Player Characters.
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq, serde::Serialize, strum::EnumIter)]
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

        #[allow(dead_code, clippy::all)]
        pub mod generate {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type HeroicCulture =
                super::super::super::loremaster::cultures::types::HeroicCulture;
            #[allow(unused_unsafe, clippy::all)]
            /// Generate a random Heroic Culture.
            pub fn generate_culture() -> HeroicCulture {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "loremaster:cultures/generate")]
                    extern "C" {
                        #[link_name = "generate-culture"]
                        fn wit_import() -> i32;
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::loremaster::cultures::types::HeroicCulture::_lift(
                        ret as u8,
                    )
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Generate a random name for a given Heroic Culture.
            pub fn generate_name(culture: HeroicCulture) -> _rt::String {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "loremaster:cultures/generate")]
                    extern "C" {
                        #[link_name = "generate-name"]
                        fn wit_import(_: i32, _: *mut u8);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(culture.clone() as i32, ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                    _rt::string_lift(bytes3)
                }
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    extern crate alloc as alloc_crate;
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:handler:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 659] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x95\x04\x01A\x02\x01\
A\x0a\x01B\x02\x01m\x06\x08bardings\x16dwarves-of-durins-folk\x0felves-of-lindon\
\x14hobbits-of-the-shire\x0bmen-of-bree\x14rangers-of-the-north\x04\0\x0eheroic-\
culture\x03\0\0\x03\x01\x19loremaster:cultures/types\x05\0\x02\x03\0\0\x0eheroic\
-culture\x01B\x04\x02\x03\x02\x01\x01\x04\0\x0eheroic-culture\x03\0\0\x01r\x02\x0e\
heroic-culture\x01\x04names\x04\0\x09character\x03\0\x02\x03\x01\x1bloremaster:c\
haracters/types\x05\x02\x02\x03\0\x01\x09character\x01B\x04\x02\x03\x02\x01\x03\x04\
\0\x09character\x03\0\0\x01@\0\0\x01\x04\0\x12generate-character\x01\x02\x03\x01\
\x1eloremaster:characters/generate\x05\x04\x01B\x06\x02\x03\x02\x01\x01\x04\0\x0e\
heroic-culture\x03\0\0\x01@\0\0\x01\x04\0\x10generate-culture\x01\x02\x01@\x01\x07\
culture\x01\0s\x04\0\x0dgenerate-name\x01\x03\x03\x01\x1cloremaster:cultures/gen\
erate\x05\x05\x04\x01\x1aloremaster:handler/handler\x04\0\x0b\x0d\x01\0\x07handl\
er\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.208.1\x10\
wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
