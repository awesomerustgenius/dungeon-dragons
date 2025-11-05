#![allow(dead_code, unused_variables)]
use crate::prelude::*;

pub const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
  pub map: Map,
  pub rooms: Vec<Rect>,
  pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rand: &mut RandomNumberGenerator) -> Self {
      let mut map_builder = MapBuilder{
        map: Map::new(),
        rooms: Vec::new(),
        player_start: Point::zero(),
      };
      map_builder.fill(TileType::Wall);
      map_builder.build_random_rooms(rand);
      map_builder.build_corridors(rand);
      map_builder.player_start = map_builder.rooms[0].center();

      map_builder
    } 
  
    fn fill(&mut self, tile: TileType) {
      self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rand: &mut RandomNumberGenerator) {
      while self.rooms.len() < NUM_ROOMS {
        // Generate random rooms.
          let rooms = Rect::with_size(
            rand.range(1, SCREEN_WIDTH - 10),
            rand.range(1, SCREEN_HEIGHT - 10),
            rand.range(2, 10), 
            rand.range(2, 10)
          );

          let mut overlap  = false;
          for room in self.rooms.iter() {
            // Check whether the rooms intersects.
            if room.intersect(&rooms) {
              overlap = true;
            }
          }
          if !overlap {
            rooms.for_each(|room| {
              // check whether the rooms are withing the map boundaries.
              if room.x > 0 && room.x < SCREEN_WIDTH && room.y > 0 && room.y < SCREEN_HEIGHT {
                let idx = map_idx(room.x, room.y);
                self.map.tiles[idx] = TileType::Floor;
              }
            });
          }
          self.rooms.push(rooms);
      }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
      use std::cmp::{min, max};
      for y in min(y1, y2) ..= max(y1, y2) {
        if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
          self.map.tiles[idx as usize] = TileType::Floor;
        } 
      }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
      use std::cmp::{min, max};
      for x in min(x1, x2) ..= max(x1, x2) {
        if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
          self.map.tiles[idx as usize] = TileType::Floor
        }
      }
    }
    
    fn build_corridors(&mut self, rand: &mut RandomNumberGenerator) {
      let mut rooms = self.rooms.clone();
      // Sorting rooms by their center points before allocating corridors. This helps to avoid long corridors that almost certainly overlap with other rooms.
      rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
      
      for (idx, room) in rooms.iter().enumerate().skip(1) {
        let prev = rooms[idx - 1].center();
        let new = room.center();

        if rand.range(0, 2) == 1 {
          self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
          self.apply_vertical_tunnel(prev.y, new.y, prev.x);
        } else {
            self.apply_vertical_tunnel(prev.y, new.y, prev.x);
            self.apply_horizontal_tunnel(prev.x, new.x, new.y);
        }
      }
    }

}