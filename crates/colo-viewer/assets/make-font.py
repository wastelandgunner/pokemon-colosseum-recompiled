"""Build PoppinsColo-Medium.ttf: Poppins Medium plus the symbols the game's text uses that Poppins lacks
(★ ☆ in P★DA, ♂ ♀ for Pokémon genders), taken from DejaVu Sans and scaled to Poppins' 1000-unit em.

    python3 make-font.py Poppins-Medium.ttf /usr/share/fonts/truetype/dejavu/DejaVuSans.ttf PoppinsColo-Medium.ttf
"""
import sys
from fontTools.ttLib import TTFont
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.pens.transformPen import TransformPen

base_path, donor_path, out_path = sys.argv[1:4]
base, donor = TTFont(base_path), TTFont(donor_path)
scale = base['head'].unitsPerEm / donor['head'].unitsPerEm
donor_cmap, donor_glyphs = donor.getBestCmap(), donor.getGlyphSet()
for cp in (0x2605, 0x2606, 0x2642, 0x2640):
    src = donor_cmap[cp]
    name = f'uni{cp:04X}'
    pen = TTGlyphPen(None)
    # DejaVu's symbols sit a bit large next to Poppins capitals: 0.85 of the em-scaled size, raised to cap height
    k = scale * 0.85
    donor_glyphs[src].draw(TransformPen(pen, (k, 0, 0, k, 0, 40)))
    base['glyf'][name] = pen.glyph()
    base['hmtx'][name] = (round(donor['hmtx'][src][0] * k), round(donor['hmtx'][src][1] * k))
    for table in base['cmap'].tables:
        if table.isUnicode():
            table.cmap[cp] = name
order = base['glyf'].glyphOrder
base.setGlyphOrder(order)
base['maxp'].numGlyphs = len(order)
for rec in base['name'].names:
    if rec.nameID in (1, 3, 4, 6, 16):
        s = rec.toUnicode()
        rec.string = s.replace('Poppins', 'Poppins Colo' if rec.nameID != 6 else 'PoppinsColo')
base.save(out_path)
print('wrote', out_path)
