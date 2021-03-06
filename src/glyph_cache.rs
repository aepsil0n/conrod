
use freetype;
use label::FontSize;
use opengl_graphics::Texture;
use piston::AssetStore;
use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

/// Struct used to hold rendered character data.
pub struct Character {
    pub glyph: freetype::Glyph,
    pub bitmap_glyph: freetype::BitmapGlyph,
    pub texture: Texture,
}

/// A struct used for caching rendered font.
pub struct GlyphCache {
    pub face: freetype::Face,
    data: HashMap<FontSize, HashMap<char, Character>>,
}

impl GlyphCache {

    /// Constructor for a GlyphCache.
    pub fn new(font_file: &str) -> GlyphCache {
        let freetype = freetype::Library::init().unwrap();
        let asset_store = AssetStore::from_folder("../assets");
        let font = asset_store.path(font_file).unwrap();
        let font_str = match font.as_str() {
            Some(font_str) => font_str,
            None => fail!("GlyphCache::new() : Failed to return `font.as_str()`."),
        };
        let face = match freetype.new_face(font_str, 0) {
            Ok(face) => face,
            Err(err) => fail!("GlyphCache::new() : {}", err),
        };
        GlyphCache {
            face: face,
            data: HashMap::new(),
        }
    }

    /// Return a reference to a `Character`. If there is not yet a `Character` for
    /// the given `FontSize` and `char`, load the `Character`.
    pub fn get_character(&mut self, size: FontSize, ch: char) -> &Character {
        match {
            match self.data.entry(size) {
                Vacant(entry) => entry.set(HashMap::new()),
                Occupied(entry) => entry.into_mut(),
            }
        }.contains_key(&ch) {
            true => &self.data[size][ch],
            false => { self.load_character(size, ch); &self.data[size][ch] }
        }
    }

    /// Load a `Character` from a given `FontSize` and `char`.
    fn load_character(&mut self, size: FontSize, ch: char) {
        self.face.set_pixel_sizes(0, size).unwrap();
        self.face.load_char(ch as u64, freetype::face::DEFAULT).unwrap();
        let glyph = self.face.glyph().get_glyph().unwrap();
        let bitmap_glyph = glyph.to_bitmap(freetype::render_mode::Normal, None).unwrap();
        let bitmap = bitmap_glyph.bitmap();
        let texture = Texture::from_memory_alpha(bitmap.buffer(),
                                                 bitmap.width() as u32,
                                                 bitmap.rows() as u32).unwrap();
        self.data.get_mut(&size).insert(ch, Character {
            glyph: glyph,
            bitmap_glyph: bitmap_glyph,
            texture: texture,
        });
    }

}

