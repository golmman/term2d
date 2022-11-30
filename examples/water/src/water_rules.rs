use term2d::point::Point;

use crate::{random::Random, world::Droplet};

pub fn setup_water_rules() -> Vec<fn(&Droplet, &mut Random) -> Droplet> {
    let mut rules: Vec<fn(&Droplet, &mut Random) -> Droplet> = vec![fall_straight; 32];

    // .x.
    // ...
    rules[0] = fall_straight;

    // .x.
    // ..#
    rules[1] = fall_straight;

    // .x.
    // .#.
    rules[2] = fall_left_or_right;

    // .x.
    // .##
    rules[3] = fall_left;

    // .x.
    // #..
    rules[4] = fall_straight;

    // .x.
    // #.#
    rules[5] = fall_straight;

    // .x.
    // ##.
    rules[6] = fall_right;

    // .x.
    // ###
    rules[7] = stay_or_move_left_or_right;

    // .x#
    // ...
    rules[8] = fall_straight;

    // .x#
    // ..#
    rules[9] = fall_straight;

    // .x#
    // .#.
    rules[10] = fall_left_or_right;

    // .x#
    // .##
    rules[11] = fall_left;

    // .x#
    // #..
    rules[12] = fall_straight;

    // .x#
    // #.#
    rules[13] = fall_straight;

    // .x#
    // ##.
    rules[14] = fall_right;

    // .x#
    // ###
    rules[15] = stay_or_move_left;

    // #x.
    // ...
    rules[16] = fall_straight;

    // #x.
    // ..#
    rules[17] = fall_straight;

    // #x.
    // .#.
    rules[18] = fall_left_or_right;

    // #x.
    // .##
    rules[19] = fall_left;

    // #x.
    // #..
    rules[20] = fall_straight;

    // #x.
    // #.#
    rules[21] = fall_straight;

    // #x.
    // ##.
    rules[22] = fall_right;

    // #x.
    // ###
    rules[23] = stay_or_move_right;

    // #x#
    // ...
    rules[24] = fall_straight;

    // #x#
    // ..#
    rules[25] = fall_straight;

    // #x#
    // .#.
    rules[26] = fall_left_or_right;

    // #x#
    // .##
    rules[27] = fall_left;

    // #x#
    // #..
    rules[28] = fall_straight;

    // #x#
    // #.#
    rules[29] = fall_straight;

    // #x#
    // ##.
    rules[30] = fall_right;

    // #x#
    // ###
    rules[31] = stay;

    rules
}

fn stay_or_move_left(d: &Droplet, r: &mut Random) -> Droplet {
    let pos = if r.next() % 20 == 0 {
        d.pos.left()
    } else {
        d.pos.clone()
    };
    Droplet {
        pos,
        vel: Point::new(0, 0),
    }
}

fn stay_or_move_right(d: &Droplet, r: &mut Random) -> Droplet {
    let pos = if r.next() % 20 == 0 {
        d.pos.right()
    } else {
        d.pos.clone()
    };
    Droplet {
        pos,
        vel: Point::new(0, 0),
    }
}

fn stay_or_move_left_or_right(d: &Droplet, r: &mut Random) -> Droplet {
    let pos = if r.next() % 20 == 0 {
        if r.next() % 2 == 0 {
            d.pos.left()
        } else {
            d.pos.right()
        }
    } else {
        d.pos.clone()
    };
    Droplet {
        pos,
        vel: Point::new(0, 0),
    }
}

fn stay(d: &Droplet, _r: &mut Random) -> Droplet {
    Droplet {
        pos: d.pos.clone(),
        vel: Point::new(0, 0),
    }
}

fn fall_left(d: &Droplet, _r: &mut Random) -> Droplet {
    Droplet {
        pos: d.pos.down_left(),
        vel: Point::new(0, 0),
    }
}

fn fall_right(d: &Droplet, _r: &mut Random) -> Droplet {
    Droplet {
        pos: d.pos.down_right(),
        vel: Point::new(0, 0),
    }
}

fn fall_left_or_right(d: &Droplet, r: &mut Random) -> Droplet {
    let pos = if r.next() % 2 == 0 {
        d.pos.down_right()
    } else {
        d.pos.down_left()
    };
    Droplet {
        pos,
        vel: Point::new(0, 0),
    }
}

fn fall_straight(d: &Droplet, _r: &mut Random) -> Droplet {
    Droplet {
        pos: d.pos.down(),
        vel: Point::new(0, 0),
    }
}
