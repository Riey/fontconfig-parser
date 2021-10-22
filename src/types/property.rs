use crate::{Bool, CharSet, Double, Int};
use enum_kinds::EnumKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, EnumKind)]
#[enum_kind(
    PropertyKind,
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum Property {
    /// Font family names
    Family(String),
    /// Languages corresponding to each family
    FamilyLang(String),
    /// Font style. Overrides weight and slant
    Style(String),
    /// Languages corresponding to each style
    StyleLang(String),
    /// Font full names (often includes style)
    FullName(String),
    /// Languages corresponding to each fullname
    FullNameLang(String),

    /// Italic, oblique or roman
    Slant(Int),
    /// Light, medium, demibold, bold or black
    Weight(Int),
    /// Point size
    Size(Double),
    /// Condensed, normal or expanded
    Width(Int),
    /// Stretches glyphs horizontally before hinting
    Aspect(Double),
    /// Pixel size
    PixelSize(Double),
    /// Proportional, dual-width, monospace or charcell
    Spacing(Int),
    /// Font foundry name
    Foundry(String),
    /// Whether glyphs can be antialiased
    Antialias(Bool),
    /// Whether the rasterizer should use hinting
    Hinting(Bool),
    /// Automatic hinting style
    HintStyle(Int),
    /// Automatic hinting style
    VerticalLayout(Bool),
    /// Use autohinter instead of normal hinter
    AutoHint(Bool),
    /// Use font global advance data (deprecated)
    GlobalAdvance(Bool),

    /// The filename holding the font
    File(String),
    /// The index of the font within the file
    Index(Int),
    // TODO:
    // /// Use the specified FreeType face object
    // Ftface(FT_Face),
    /// Which rasterizer is in use (deprecated)
    Rasterizer(String),
    /// Whether the glyphs are outlines
    Outline(Bool),
    /// Whether glyphs can be scaled
    Scalable(Bool),
    /// Whether any glyphs have color
    Color(Bool),
    /// Scale factor for point->pixel conversions (deprecated)
    Scale(Double),
    /// Target dots per inch
    Dpi(Double),
    /// unknown, rgb, bgr, vrgb, vbgr, none - subpixel geometry
    Rgba(Int),
    /// Type of LCD filter
    Lcdfilter(Int),
    /// Eliminate leading from line spacing
    Minspace(Bool),
    /// Unicode chars encoded by the font
    Charset(CharSet),
    /// List of RFC-3066-style languages this font supports
    Lang(String),
    /// Version number of the font
    Fontversion(Int),
    /// List of layout capabilities in the font
    Capability(String),
    /// String name of the font format
    Fontformat(String),
    /// Rasterizer should synthetically embolden the font
    Embolden(Bool),
    /// Use the embedded bitmap instead of the outline
    Embeddedbitmap(Bool),
    /// Whether the style is a decorative variant
    Decorative(Bool),
    /// List of the feature tags in OpenType to be enabled
    Fontfeatures(String),
    /// Language name to be used for the default value of familylang, stylelang, and fullnamelang
    Namelang(String),
    /// String  Name of the running program
    Prgname(String),
    /// Font family name in PostScript
    Postscriptname(String),
    /// Whether the font has hinting
    Fonthashint(Bool),
    /// Order number of the font
    Order(Int),
}
