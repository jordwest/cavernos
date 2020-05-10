use crate::cavernos::CavernOS;
use crate::geometry::Vec2d;
use rand::prelude::*;

#[derive(Clone, Copy)]
enum MapTile {
    Rock,
    Floor,
}

struct Room {
    pos: Vec2d<isize>,
    size: Vec2d<isize>,
}
impl Room {
    fn midpoint(&self) -> Vec2d<isize> {
        self.pos + (self.size / 2)
    }
}

pub struct RoguelikeState {
    map: [MapTile; 65536],
    seen: [bool; 65536],
    width: isize,
    height: isize,
    rng: rand::rngs::StdRng,
    player_pos: Vec2d<isize>,
    rendered: bool,
    rooms: Vec<Room>,
}

// Input addresses as specified in manifest.json
const MOVE_UP: usize = 9;
const MOVE_DOWN: usize = 10;
const MOVE_LEFT: usize = 11;
const MOVE_RIGHT: usize = 12;

impl RoguelikeState {
    pub fn new() -> RoguelikeState {
        let mut state = RoguelikeState {
            map: [MapTile::Rock; 65536],
            seen: [false; 65536],
            width: 255,
            height: 255,
            rng: rand::rngs::StdRng::seed_from_u64(67852322),
            rooms: Vec::with_capacity(200),
            player_pos: Vec2d { x: 128, y: 128 },
            rendered: false,
        };

        state.carve_room(120, 124, 10, 9);
        state.carve_room(108, 118, 4, 6);
        state.carve_hallway(124, 127, 110, 124);

        let initial_room = Room {
            pos: Vec2d { x: 122, y: 124 },
            size: Vec2d { x: 7, y: 5 },
        };
        state.rooms.push(initial_room);

        for _ in 1..=state.rooms.capacity() {
            let size = Vec2d {
                x: state.rng.gen_range(3, 7),
                y: state.rng.gen_range(3, 7),
            };
            let pos = Vec2d {
                x: state.rng.gen_range(0, state.width - size.x),
                y: state.rng.gen_range(0, state.height - size.y),
            };
            let room = Room { size, pos };
            state.carve_room(room.pos.x, room.pos.y, room.size.x, room.size.y);

            // Find a nearby room to join with
            let midpoint = room.midpoint();
            let mut closest_room = None;
            let room_count = state.rooms.len();
            for i in 0..room_count {
                let dist = (state.rooms[i].midpoint() - midpoint).len();
                closest_room = match closest_room {
                    None => Some((dist, i)),
                    Some((current_min_dist, _)) if dist < current_min_dist => Some((dist, i)),
                    existing_val => existing_val,
                }
            }

            if let Some((_, other_room_i)) = closest_room {
                let other_room_pos = state.rooms[other_room_i].pos.clone();
                state.carve_hallway(room.pos.x, room.pos.y, other_room_pos.x, other_room_pos.y);
            }
            state.rooms.push(room);
        }

        state
    }

    pub fn carve_room(&mut self, left: isize, top: isize, width: isize, height: isize) {
        let right = left + width;
        let bottom = top + height;
        for y in top..bottom {
            for x in left..right {
                self.map[(y * self.width + x) as usize] = MapTile::Floor;
            }
        }
    }

    fn tile_at(&self, x: isize, y: isize) -> MapTile {
        let i = ((y * self.width) + x) as usize;
        self.map[i]
    }

    fn set_seen(&mut self, x: isize, y: isize) {
        let i = ((y * self.width) + x) as usize;
        self.seen[i] = true;
    }
    fn has_seen(&self, x: isize, y: isize) -> bool {
        let i = ((y * self.width) + x) as usize;
        self.seen[i]
    }

    fn set_tile_at(&mut self, x: isize, y: isize, tile: MapTile) {
        let i = ((y * self.width) + x) as usize;
        self.map[i] = tile;
    }

    fn carve_hallway(&mut self, start_x: isize, start_y: isize, end_x: isize, end_y: isize) {
        let mut x = start_x;
        let mut y = start_y;

        while x != end_x {
            if x < end_x {
                x += 1;
            } else {
                x -= 1;
            }
            self.set_tile_at(x, y, MapTile::Floor);
        }

        while y != end_y {
            if y < end_y {
                y += 1;
            } else {
                y -= 1;
            }
            self.set_tile_at(x, y, MapTile::Floor);
        }
    }

    fn map_coord_to_screen(&self, os: &mut CavernOS, map: Vec2d<isize>) -> Option<Vec2d<u8>> {
        // How many tiles can we fit on screen?
        let cols = (os.config.cols / 2) as isize; // Divide by two since we're rendering square tiles (which take two cells)
        let rows = os.config.rows as isize;

        // Find top left of screen in map relative coord
        let first_col = (self.player_pos.x as isize) - (cols / 2);
        let first_row = (self.player_pos.y as isize) - (rows / 2);

        if map.x > first_col + cols
            || map.x < first_col
            || map.y > first_row + rows
            || map.y < first_row
        {
            return None;
        }

        Some(Vec2d {
            x: ((map.x - first_col) * 2) as u8,
            y: (map.y - first_row) as u8,
        })
    }

    /// Draw a character to screen using map relative coordinates
    fn set_char(&self, os: &mut CavernOS, x: isize, y: isize, c: u8) {
        let pos = self.map_coord_to_screen(os, Vec2d { x, y });
        if let Some(pos) = pos {
            os.set_char(pos.x, pos.y, 255);
            os.set_char(pos.x + 1, pos.y, c);
        }
    }

    fn set_fg_color(&self, os: &mut CavernOS, x: isize, y: isize, c: u8) {
        let pos = self.map_coord_to_screen(os, Vec2d { x, y });
        if let Some(pos) = pos {
            os.set_fg_color(pos.x, pos.y, c);
            os.set_fg_color(pos.x + 1, pos.y, c);
        }
    }

    fn set_bg_color(&self, os: &mut CavernOS, x: isize, y: isize, c: u8) {
        let pos = self.map_coord_to_screen(os, Vec2d { x, y });
        if let Some(pos) = pos {
            os.set_bg_color(pos.x, pos.y, c);
            os.set_bg_color(pos.x + 1, pos.y, c);
        }
    }

    fn calculate_light_at(&mut self, x: isize, y: isize) -> f32 {
        use bresenham::Bresenham;
        let pos = Vec2d { x, y };
        let distance_from_player = (pos - self.player_pos).len();
        if distance_from_player > 8.0 {
            0.0
        } else {
            // Step from the player to this position to check for obstructions
            let mut light_level = 8;
            for (x, y) in Bresenham::new(
                (self.player_pos.x as isize, self.player_pos.y as isize),
                (pos.x as isize, pos.y as isize),
            ) {
                if light_level <= 0 {
                    return 0.;
                }

                if let MapTile::Rock = self.tile_at(x as isize, y as isize) {
                    return 0.;
                }

                light_level -= 1;
            }
            if light_level > 0 {
                self.set_seen(x, y);
            }
            return (light_level as f32) / 8.0;
        }
    }

    fn inside_bounds(&self, pos: Vec2d<isize>) -> bool {
        pos.x < self.width && pos.x >= 0 && pos.y < self.height && pos.y >= 0
    }

    fn process_inputs(&mut self, os: &mut CavernOS) {
        let mut move_by: Option<Vec2d<isize>> = None;
        if os.get_input_latch(MOVE_UP) {
            // Move up
            move_by = Some(Vec2d { x: 0, y: -1 });
        }
        if os.get_input_latch(MOVE_DOWN) {
            // Move down
            move_by = Some(Vec2d { x: 0, y: 1 });
        }
        if os.get_input_latch(MOVE_LEFT) {
            // Move left
            move_by = Some(Vec2d { x: -1, y: 0 });
        }
        if os.get_input_latch(MOVE_RIGHT) {
            // Move right
            move_by = Some(Vec2d { x: 1, y: 0 });
        }

        if let Some(move_by) = move_by {
            let new_pos = self.player_pos + move_by;

            match self.tile_at(new_pos.x, new_pos.y) {
                MapTile::Rock => (),
                MapTile::Floor => {
                    if self.inside_bounds(new_pos) {
                        self.player_pos = new_pos
                    }
                }
            }
        }
    }
}

pub fn frame(os: &mut CavernOS, state: &mut RoguelikeState, _dt: f64) {
    state.process_inputs(os);

    // How many tiles can we fit on screen?
    let cols = (os.config.cols / 2) as isize; // Divide by two since we're rendering square tiles (which take two cells)
    let rows = os.config.rows as isize;

    // The first tile we'll render in the top left (calculated by positioning the player in the
    // centre)
    let first_col = (state.player_pos.x) - (cols / 2);
    let first_row = (state.player_pos.y) - (rows / 2);

    for y in 0..rows {
        for x in 0..cols {
            let cell_x = (x * 2) as u8;
            let cell_y = y as u8;
            let tile_x = first_col + x;
            let tile_y = first_row + y;

            if !state.inside_bounds(Vec2d {
                x: tile_x,
                y: tile_y,
            }) {
                continue;
            }

            let tile = state.tile_at(tile_x, tile_y);

            let c = match tile {
                MapTile::Rock => {
                    let surrounding_tiles = (
                        state.tile_at(tile_x - 1, tile_y),
                        state.tile_at(tile_x + 1, tile_y),
                        state.tile_at(tile_x, tile_y - 1),
                        state.tile_at(tile_x, tile_y + 1),
                        state.tile_at(tile_x - 1, tile_y - 1),
                        state.tile_at(tile_x + 1, tile_y - 1),
                        state.tile_at(tile_x - 1, tile_y + 1),
                        state.tile_at(tile_x + 1, tile_y + 1),
                    );
                    match surrounding_tiles {
                        // Nothing but rock, don't render this rock
                        (
                            MapTile::Rock,
                            MapTile::Rock,
                            MapTile::Rock,
                            MapTile::Rock,
                            MapTile::Rock,
                            MapTile::Rock,
                            MapTile::Rock,
                            MapTile::Rock,
                        ) => 0,

                        // Something else nearby, so render this rock as a wall
                        _ => 35,
                    }
                }
                MapTile::Floor => 46,
            };
            os.set_char(cell_x, cell_y, 255);
            os.set_char(cell_x + 1, cell_y, c);
            let light = state.calculate_light_at(tile_x, tile_y);

            if light > 0.0 {
                let light_color = match state.tile_at(tile_x, tile_y) {
                    MapTile::Floor => 99,
                    MapTile::Rock => 104,
                };
                os.set_bg_color(cell_x, cell_y, light_color);
                os.set_bg_color(cell_x + 1, cell_y, light_color);
                os.set_fg_color(cell_x, cell_y, 110);
                os.set_fg_color(cell_x + 1, cell_y, 110);
            } else if state.has_seen(tile_x, tile_y) {
                os.set_bg_color(cell_x, cell_y, 0);
                os.set_bg_color(cell_x + 1, cell_y, 0);
                os.set_fg_color(cell_x, cell_y, 243);
                os.set_fg_color(cell_x + 1, cell_y, 243);
            } else {
                state.set_bg_color(os, tile_x, tile_y, 0);
                state.set_fg_color(os, tile_x, tile_y, 0);
            }
        }
    }

    state.set_char(os, state.player_pos.x, state.player_pos.y, 64);
    state.set_fg_color(os, state.player_pos.x, state.player_pos.y, 124);
    state.set_bg_color(os, state.player_pos.x, state.player_pos.y, 109);
    state.rendered = true;
}
