pub mod fov {
    //! taken from
    //! <http://adammil.net/blog/v125_Roguelike_Vision_Algorithms.html#mycode>
    use crate::components::Position;
    use bevy::utils::hashbrown::HashSet;

    /// represents the slope Y/X as a rational number
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Slope {
        y: u32,
        x: u32,
    }

    impl Slope {
        fn new(y: u32, x: u32) -> Self {
            Self { y, x }
        }

        fn greater(&self, y: u32, x: u32) -> bool {
            self.y * x > self.x * y
        }

        fn greater_or_equal(&self, y: u32, x: u32) -> bool {
            self.y * x >= self.x * y
        }

        fn less(&self, y: u32, x: u32) -> bool {
            self.y * x < self.x * y
        }

        //public bool LessOrEqual(uint y, uint x) { return Y*x <= X*y; } // this <= y/x
    }

    /// # Args
    /// * `blocks_light` - A function that accepts the X and Y coordinates of a tile and determines whether the given tile blocks the passage of light. The function must be able to accept coordinates that are out of bounds.
    /// * `set_visible` - A function that sets a tile to be visible, given its X and Y coordinates. The function must ignore coordinates that are out of bounds.
    /// * `get_distance` - A function that takes the X and Y coordinate of a point where X >= 0, Y >= 0, and X >= Y, and returns the distance from the point to the origin (0,0).
    pub struct MyVisibility<F, /* G, */ H>
    where
        F: Fn(i32, i32) -> bool,
        // G: FnMut(i32, i32),
        H: Fn(i32, i32) -> i32,
    {
        blocks_light: F,
        // set_visible: G,
        get_distance: H,
    }

    impl<F, /* G, */ H> MyVisibility<F, /* G, */ H>
    where
        F: Fn(i32, i32) -> bool,
        // G: Fn(i32, i32),
        H: Fn(i32, i32) -> i32,
    {
        pub fn new(blocks_light: F, /*  set_visible: G, */ get_distance: H) -> Self {
            Self {
                blocks_light,
                // set_visible,
                get_distance,
            }
        }

        pub fn compute(&self, origin: Position, range_limit: i32) -> HashSet<Position> {
            let mut visible_pos = HashSet::new();
            visible_pos.insert(origin);

            for octant in 0..8 {
                self.compute_octant(
                    octant,
                    &origin,
                    range_limit,
                    1,
                    Slope::new(1, 1),
                    Slope::new(0, 1),
                    &mut visible_pos,
                );
            }
            visible_pos
        }

        // throughout this function there are references to various parts of tiles. A tile's coordinates refer to its
        // center, and the following diagram shows the parts of the tile and the vectors from the origin that pass through
        // those parts. given a part of a tile with vector u, a vector v passes above it if v > u and below it if v < u
        //    g         center:        y / x
        // a------b   a top left:      (y*2+1) / (x*2-1)   i inner top left:      (y*4+1) / (x*4-1)
        // |  /\  |   b top right:     (y*2+1) / (x*2+1)   j inner top right:     (y*4+1) / (x*4+1)
        // |i/__\j|   c bottom left:   (y*2-1) / (x*2-1)   k inner bottom left:   (y*4-1) / (x*4-1)
        //e|/|  |\|f  d bottom right:  (y*2-1) / (x*2+1)   m inner bottom right:  (y*4-1) / (x*4+1)
        // |\|__|/|   e middle left:   (y*2) / (x*2-1)
        // |k\  /m|   f middle right:  (y*2) / (x*2+1)     a-d are the corners of the tile
        // |  \/  |   g top center:    (y*2+1) / (x*2)     e-h are the corners of the inner (wall) diamond
        // c------d   h bottom center: (y*2-1) / (x*2)     i-m are the corners of the inner square (1/2 tile width)
        //    h
        fn compute_octant(
            &self,
            octant: u32,
            origin: &Position,
            range_limit: i32,
            x: u32,
            top: Slope,
            bottom: Slope,
            visible_pos: &mut HashSet<Position>,
        ) {
            let mut x = x;
            let mut top = top;
            let mut bottom = bottom;

            while x <= range_limit as u32 {
                // compute the Y coordinates of the top and bottom of the sector. we maintain that top > bottom
                let mut top_y;

                // if top == ?/1 then it must be 1/1 because 0/1 < top <= 1/1. this is special-cased because top
                // starts at 1/1 and remains 1/1 as long as it doesn't hit anything, so it's a common case
                if top.x == 1 {
                    top_y = x;
                } else {
                    // top < 1
                    // get the tile that the top vector enters from the left. since our coordinates refer to the center of the
                    // tile, this is (x-0.5)*top+0.5, which can be computed as (x-0.5)*top+0.5 = (2(x+0.5)*top+1)/2 =
                    // ((2x+1)*top+1)/2. since top == a/b, this is ((2x+1)*a+b)/2b. if it enters a tile at one of the left
                    // corners, it will round up, so it'll enter from the bottom-left and never the top-left
                    top_y = ((x * 2 - 1) * top.y + top.x) / (top.x * 2);

                    // now it's possible that the vector passes from the left side of the tile up into the tile above before
                    // exiting from the right side of this column. so we may need to increment topY

                    // if the tile blocks light (i.e. is a wall)...
                    if (self.blocks_light)(
                        self.translate_x(octant, x, top_y, origin),
                        self.translate_y(octant, x, top_y, origin),
                    ) {
                        // if the tile entered from the left blocks light, whether it passes into the tile above depends on the shape
                        // of the wall tile as well as the angle of the vector. if the tile has does not have a beveled top-left
                        // corner, then it is blocked. the corner is beveled if the tiles above and to the left are not walls. we can
                        // ignore the tile to the left because if it was a wall tile, the top vector must have entered this tile from
                        // the bottom-left corner, in which case it can't possibly enter the tile above.
                        //
                        // otherwise, with a beveled top-left corner, the slope of the vector must be greater than or equal to the
                        // slope of the vector to the top center of the tile (x*2, topY*2+1) in order for it to miss the wall and
                        // pass into the tile above
                        if top.greater_or_equal(top_y * 2 + 1, x * 2)
                            && !(self.blocks_light)(
                                self.translate_x(octant, x, top_y + 1, origin),
                                self.translate_y(octant, x, top_y + 1, origin),
                            )
                        {
                            top_y += 1;
                        }
                    }
                    // the tile doesn't block light
                    else if top.greater(top_y * 2 + 1, x * 2 + 1)
                        && (self.blocks_light)(
                            self.translate_x(octant, x + 1, top_y, origin),
                            self.translate_y(octant, x + 1, top_y, origin),
                        )
                    {
                        // since this tile doesn't block light, there's nothing to stop it from passing into the tile above, and it
                        // does so if the vector is greater than the vector for the bottom-right corner of the tile above. however,
                        // there is one additional consideration. later code in this method assumes that if a tile blocks light then
                        // it must be visible, so if the tile above blocks light we have to make sure the light actually impacts the
                        // wall shape. now there are three cases: 1) the tile above is clear, in which case the vector must be above
                        // the bottom-right corner of the tile above, 2) the tile above blocks light and does not have a beveled
                        // bottom-right corner, in which case the vector must be above the bottom-right corner, and 3) the tile above
                        // blocks light and does have a beveled bottom-right corner, in which case the vector must be above the
                        // bottom center of the tile above (i.e. the corner of the beveled edge).
                        //
                        // now it's possible to merge 1 and 2 into a single check, and we get the following: if the tile above and to
                        // the right is a wall, then the vector must be above the bottom-right corner. otherwise, the vector must be
                        // above the bottom center. this works because if the tile above and to the right is a wall, then there are
                        // two cases: 1) the tile above is also a wall, in which case we must check against the bottom-right corner,
                        // or 2) the tile above is not a wall, in which case the vector passes into it if it's above the bottom-right
                        // corner. so either way we use the bottom-right corner in that case. now, if the tile above and to the right
                        // is not a wall, then we again have two cases: 1) the tile above is a wall with a beveled edge, in which
                        // case we must check against the bottom center, or 2) the tile above is not a wall, in which case it will
                        // only be visible if light passes through the inner square, and the inner square is guaranteed to be no
                        // larger than a wall diamond, so if it wouldn't pass through a wall diamond then it can't be visible, so
                        // there's no point in incrementing topY even if light passes through the corner of the tile above. so we
                        // might as well use the bottom center for both cases.
                        top_y += 1;
                    }
                }

                let mut bottom_y;

                // if bottom == 0/?, then it's hitting the tile at Y=0 dead center. this is special-cased because
                // bottom.Y starts at zero and remains zero as long as it doesn't hit anything, so it's common
                if bottom.y == 0 {
                    bottom_y = 0;
                }
                // bottom > 0
                else {
                    // the tile that the bottom vector enters from the left
                    bottom_y = ((x * 2 - 1) * bottom.y + bottom.x) / (bottom.x * 2);

                    // code below assumes that if a tile is a wall then it's visible, so if the tile contains a wall we have to
                    // ensure that the bottom vector actually hits the wall shape. it misses the wall shape if the top-left corner
                    // is beveled and bottom >= (bottomY*2+1)/(x*2). finally, the top-left corner is beveled if the tiles to the
                    // left and above are clear. we can assume the tile to the left is clear because otherwise the bottom vector
                    // would be greater, so we only have to check above
                    if bottom.greater_or_equal(bottom_y * 2 + 1, x * 2)
                        && (self.blocks_light)(
                            self.translate_x(octant, x, bottom_y, origin),
                            self.translate_y(octant, x, bottom_y, origin),
                        )
                        && !(self.blocks_light)(
                            self.translate_x(octant, x, bottom_y + 1, origin),
                            self.translate_y(octant, x, bottom_y + 1, origin),
                        )
                    {
                        bottom_y += 1;
                    }
                }

                // go through the tiles in the column now that we know which ones could possibly be visible
                let mut was_opaque = -1; // 0:false, 1:true, -1:not applicable

                for y in (bottom_y..=top_y).rev() {
                    if range_limit < 0 || (self.get_distance)(x as i32, y as i32) <= range_limit {
                        let is_opaque = (self.blocks_light)(
                            self.translate_x(octant, x, y, origin),
                            self.translate_y(octant, x, y, origin),
                        );

                        // every tile where topY > y > bottomY is guaranteed to be visible. also, the code that initializes topY and
                        // bottomY guarantees that if the tile is opaque then it's visible. so we only have to do extra work for the
                        // case where the tile is clear and y == topY or y == bottomY. if y == topY then we have to make sure that
                        // the top vector is above the bottom-right corner of the inner square. if y == bottomY then we have to make
                        // sure that the bottom vector is below the top-left corner of the inner square
                        let is_visible = is_opaque
                            || (y != top_y || top.greater(y * 4 - 1, x * 4 + 1))
                                && (y != bottom_y || bottom.less(y * 4 + 1, x * 4 - 1));

                        // NOTE: if you want the algorithm to be either fully or mostly symmetrical, replace the line above with the
                        // following line (and uncomment the Slope.LessOrEqual method). the line ensures that a clear tile is visible
                        // only if there's an unobstructed line to its center. if you want it to be fully symmetrical, also remove
                        // the "isOpaque ||" part and see NOTE comments further down
                        // bool isVisible = isOpaque || ((y != topY || top.GreaterOrEqual(y, x)) && (y != bottomY || bottom.LessOrEqual(y, x)));
                        if is_visible {
                            visible_pos.insert(Position::new(
                                self.translate_x(octant, x, y, origin),
                                self.translate_y(octant, x, y, origin),
                                0,
                            ));
                        }

                        // if we found a transition from clear to opaque or vice versa, adjust the top and bottom vectors
                        // but don't bother adjusting them if this is the last column anyway
                        if x != range_limit as u32 {
                            if is_opaque {
                                // if we found a transition from clear to opaque, this sector is done in this column,
                                // so adjust the bottom vector upward and continue processing it in the next column
                                // if the opaque tile has a beveled top-left corner, move the bottom vector up to the top center.
                                // otherwise, move it up to the top left. the corner is beveled if the tiles above and to the left are
                                // clear. we can assume the tile to the left is clear because otherwise the vector would be higher, so
                                // we only have to check the tile above
                                if was_opaque == 0 {
                                    // top center by default
                                    let nx = x * 2;
                                    let ny = y * 2 + 1;

                                    if top.greater(ny, nx) {
                                        // if we're at the bottom of the column, then just adjust the current sector rather than recursing
                                        // since there's no chance that this sector can be split in two by a later transition back to clear
                                        if y == bottom_y {
                                            bottom = Slope::new(ny, nx);
                                            break;
                                        } else {
                                            self.compute_octant(
                                                octant,
                                                origin,
                                                range_limit,
                                                x + 1,
                                                top,
                                                Slope::new(ny, nx),
                                                visible_pos,
                                            );
                                        }
                                    }
                                    // the new bottom is greater than or equal to the top, so the new sector is empty and we'll ignore
                                    // it. if we're at the bottom of the column, we'd normally adjust the current sector rather than
                                    else {
                                        // recursing, so that invalidates the current sector and we're done
                                        if y == bottom_y {
                                            return;
                                        }
                                    }
                                }
                                was_opaque = 1;
                            } else {
                                // if we found a transition from opaque to clear, adjust the top vector downwards
                                if was_opaque > 0 {
                                    // if the opaque tile has a beveled bottom-right corner, move the top vector down to the bottom center.
                                    // otherwise, move it down to the bottom right. the corner is beveled if the tiles below and to the right
                                    // are clear. we know the tile below is clear because that's the current tile, so just check to the right
                                    let nx = x * 2;
                                    let ny = y * 2 + 1;
                                    // if y == bottom_y {
                                    //     return visible_pos;
                                    // }
                                    if bottom.greater_or_equal(ny, nx) {
                                        return;
                                    }
                                    top = Slope::new(ny, nx);
                                }
                                was_opaque = 0;
                            }
                        }
                    }
                }

                if was_opaque != 0 {
                    break;
                }
                x += 1;
            }
        }

        fn translate_x(&self, octant: u32, x: u32, y: u32, origin: &Position) -> i32 {
            match octant {
                0 => origin.x + x as i32,
                1 => origin.x + y as i32,
                2 => origin.x - y as i32,
                3 => origin.x - x as i32,
                4 => origin.x - x as i32,
                5 => origin.x - y as i32,
                6 => origin.x + y as i32,
                7 => origin.x + x as i32,
                _ => unreachable!(),
            }
        }

        fn translate_y(&self, octant: u32, x: u32, y: u32, origin: &Position) -> i32 {
            match octant {
                0 => origin.y - y as i32,
                1 => origin.y - x as i32,
                2 => origin.y - x as i32,
                3 => origin.y - y as i32,
                4 => origin.y + y as i32,
                5 => origin.y + x as i32,
                6 => origin.y + x as i32,
                7 => origin.y + y as i32,
                _ => unreachable!(),
            }
        }
    }

    // fn main() {
    //     let origin = LevelPoint { x: 5, y: 5 };
    //     let visibility = MyVisibility::new(
    //         |x, y| x < 0 || y < 0 || x > 10 || y > 10,
    //         |x, y| println!("Visible: ({}, {})", x, y),
    //         |x, y| ((x * x + y * y) as f32).sqrt() as i32,
    //     );
    //     visibility.compute(origin, 10);
    // }
}
