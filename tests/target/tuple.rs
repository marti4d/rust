// Test tuple litterals

fn foo() {
    let a = (a, a, a, a, a);
    let aaaaaaaaaaaaaaaa = (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, aaaaaaaaaaaaaa, aaaaaaaaaaaaaa);
    let aaaaaaaaaaaaaaaaaaaaaa = (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                                  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
                                  aaaaaaaaaaaaaaaaaaaaaaaaa,
                                  aaaa);
    let a = (a,);

    let b = (// This is a comment
             b, // Comment
             b /* Trailing comment */);
}

fn a() {
    ((aaaaaaaa,
      aaaaaaaaaaaaa,
      aaaaaaaaaaaaaaaaa,
      aaaaaaaaaaaaaa,
      aaaaaaaaaaaaaaaa,
      aaaaaaaaaaaaaa),)
}

fn b() {
    ((bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb),
     bbbbbbbbbbbbbbbbbb)
}

fn issue550() {
    self.visitor.visit_volume(self.level.sector_id(sector),
                              (floor_y,
                               if is_sky_flat(ceil_tex) {
                                   from_wad_height(self.height_range.1)
                               } else {
                                   ceil_y
                               }));
}
