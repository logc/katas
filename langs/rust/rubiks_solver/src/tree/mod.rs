use std::fmt;
use std::collections::LinkedList;
use std::cell::RefCell;
use cube;
use config;

mod opts;


/* brutal force cost...

    for cubes 4x4x4

    bfc(depth) = moves

    bfc(1) = 24
    bfc(2) = 24 + 24^2
    bfc(3) = bfc(3-1) + 24^3
    ...
    bfc(n) = bfc(n-1) + 24^n

    bfc(n) = bfc(n-2) + 24^(n-1) + 24^n
    bfc(n) = bfc(n-3) + 24^(n-2) + 24^(n-1) + 24^(n-0)

    bfc(n) = 24(1+24*(1+24*(1+24*...)))

*/



#[derive(Debug, Clone, Copy)]
pub struct Found {
    depth       : u8,
    iterations  : u64,
}

fn get_better(l : Option<Found>, r : &Found) -> Option<Found> {
    match l {
        Some(found_l) => {
            if found_l.iterations < r.iterations { Some(found_l) }
            else { Some(*r) }
        },
        None        => Some(*r)
    }
}


impl fmt::Display for Found {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "found.depth:  {}\n", self.depth));
        try!(write!(f, "found.iter:   {}", self.iterations));
        write!(f, "")
    }
}


#[derive(Debug, Clone, Copy)]
pub struct RotationPosition {
    pub rot     :   cube::rot::Item,
    pub position:   cube::Sides,
}



#[derive(Debug, Clone)]
pub struct PunningStats {
    pub depth_less_found        : u64,
    pub punning_3_consecutives  : u64,
    pub repeated_in_path        : u64,
    pub direction_higher_level  : u64,
    pub inverse_move            : u64,
}

fn empty_punning_stats () -> PunningStats {
    PunningStats {
                            depth_less_found: 0,
                            punning_3_consecutives: 0,
                            repeated_in_path: 0,
                            direction_higher_level:0,
                            inverse_move: 0,
                        }
}


//#[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
pub struct Status {
    pub depth           : u8,
    pub max_depth       : u8,
    pub iterations      : u64,

    pub best_found      : Option<Found>,

    current_path        : LinkedList<RotationPosition>,
    pub best_solution   : LinkedList<RotationPosition>,

    pub punning_stats   : RefCell<PunningStats>,
}


impl Status {
    fn push(&mut self, rot_pos: &RotationPosition) -> &mut Status {
        self.iterations += 1;
        self.depth += 1;
        self.current_path.push_back(*rot_pos);
        self
    }

    fn pop(&mut self) -> &mut Status {
        self.current_path.pop_back();
        self.depth -= 1;
        self
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.best_found {
            Some(best_found)    =>  try!(write!(f, "---------\nbest_found:  {}\n------------\n", best_found)),
            None                =>  try!(write!(f, "NOT FOUND\n")),
        }
        try!(write!(f, "current depth:  {}\n", self.depth));
        try!(write!(f, "iterations:     {}\n", self.iterations));
        try!(write!(f, "max_depth:      {}\n", self.max_depth));
        write!(f, "")
    }
}




pub fn explore(origin : &cube::Sides, end : &cube::Sides, max_depth : u8) -> Box<Status>
{
    let mut status  = Box::new(Status{      depth:              0,
                                            max_depth:          max_depth,
                                            iterations:         0,
                                            best_found:         None,
                                            current_path:       LinkedList::new(),
                                            best_solution:      LinkedList::new(),
                                            punning_stats:      RefCell::new(empty_punning_stats()),
                                        });
    internal_explore(origin, end, &mut status);
    status
}

//fn internal_explore<'a>(origin : &cube::Sides, end : &cube::Sides, status : &'a mut Status) -> &'a mut Status
fn internal_explore(origin : &cube::Sides, end : &cube::Sides, status : &mut Status) -> ()
{
    //println!("depth: {}", status.depth);
    //println!("depth: {}", origin);
    //println!("current_path: {:?}\n", status.shared_current_path);

    if cube::equivalent_end(origin, end) {
        //println!("current best solution {}", status.best_solution.len());
        //println!("new best solution {}  {}", status.current_path.len(), status.depth);
        {
            let update_best_solution =  |status: &mut Status|  -> () {
                status.best_found = Some(
                                                Found{ depth:   status.depth,
                                                iterations:     status.iterations, });
                status.best_solution.clear();
                for path in status.current_path.iter() {
                    status.best_solution.push_back(*path);
                }
            };

            match status.best_found {
                Some(prev_best_found)     => {
                    if status.depth < prev_best_found.depth  {
                        update_best_solution(status);
                    }
                }
                None            => update_best_solution(status),
            }
        }
        //println!("Found...... {}\n", result);
        //println!("Found moves {}\n", result.shared_best_solution.borrow().len());
        //println!("Found moves {}\n", result.shared_current_path.borrow().len());
        //println!("{}\n", &origin);
    } else {
        if status.depth < status.max_depth {
            let mut iterate_orient_dir = |  orientation : cube::rot::Orient,
                                            direction : cube::rot::Dir| -> () {
                for i in 0.. config::SIZE {
                    let next_move = &cube::rot::Item(
                            orientation,
                            direction,
                            i);

                    if opts::before_move::depth_bigger_or_equal_best_sol(status, &mut status.punning_stats.borrow_mut()) {
                        break;
                    }
                    if opts::before_move::inverse_move(&next_move, status, &mut status.punning_stats.borrow_mut()) {
                        continue;
                    }
                    if opts::before_move::three_consecutive_moves(&next_move, status, &mut status.punning_stats.borrow_mut()) {
                        continue;
                    }
                    if opts::before_move::same_direction_higher_level(&next_move, status, &mut status.punning_stats.borrow_mut()) {
                        continue;
                    }

                    let next = origin.get_rotation(next_move);
                    let rot_pos = RotationPosition{ rot: *next_move, position: next };

                    /*if opts::after_move::pos_equal2current_path(&next, status, &mut status.punning_stats.borrow_mut()) {
                        continue;
                    }*/

                    internal_explore(&next, end, status.push(&rot_pos));
                    match status.best_found {
                        Some(located_best_found)    => status.best_found = get_better(status.best_found, &located_best_found),
                        None                        => (),
                    };
                    status.pop();
                }
            };

            {
                use cube::rot::{Orient, Dir};
                iterate_orient_dir(Orient::Horizontal,  Dir::Plus);
                iterate_orient_dir(Orient::Horizontal,  Dir::Minus);

                iterate_orient_dir(Orient::Vertical,    Dir::Plus);
                iterate_orient_dir(Orient::Vertical,    Dir::Minus);

                iterate_orient_dir(Orient::Front,       Dir::Plus);
                iterate_orient_dir(Orient::Front,       Dir::Minus);
            }
        }
    }
}


/*
#[test]
fn test_tree_simple() {
    let init : cube::Sides = cube::create_from_strings(
                     ["0000",
                      "0000",
                      "0000",
                      "0000",
                 "0001 0000 0000",
                 "0002 0000 0000",
                 "0003 0000 0000",
                 "0004 0000 0000",
                      "0000",
                      "0000",
                      "0000",
                      "0000",

                      "0000",
                      "0000",
                      "0000",
                      "0000"]
        );

    let end : cube::Sides = cube::create_from_strings(
                    ["0000",
                     "0000",
                     "0000",
                     "4321",
                "0000 0000 0000",
                "0000 0000 0000",
                "0000 0000 0000",
                "0000 0000 0000",
                     "0000",
                     "0000",
                     "0000",
                     "0000",

                     "0000",
                     "0000",
                     "0000",
                     "0000"]
                );

}
*/
