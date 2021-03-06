use std::fmt;
use side;
use config;
use super::Sides;



#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Orient {
    Horizontal,
    Vertical,
    Front
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir { Plus, Minus }

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Item(pub Orient, pub Dir, pub usize);     //  ups!!! (usize)


pub fn process(sides: &Sides, item : &Item) -> Sides {
    match *item {
        Item(Orient::Horizontal, dir, level) =>    horizontal(sides, dir, level),
        Item(Orient::Vertical, dir, level)   =>    vertical  (sides, dir, level),
        Item(Orient::Front, dir, level)      =>    front     (sides, dir, level),
    }
}


impl Item {
    pub fn get_reverse(&self) -> Item {
        let Item(orientation, dir, pos) = *self;
        Item(
            orientation,
            match dir {
                Dir::Plus => Dir::Minus,
                Dir::Minus => Dir::Plus,
            },
            pos,
        )
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Item(orient, dir, pos) = *self;
        write!(f, "{}{}{}", orient, pos, dir,)
    }
}


impl fmt::Display for Orient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s_orient = match *self {
            Orient::Front        => "F",
            Orient::Horizontal   => "H",
            Orient::Vertical     => "V"
        };
        write!(f, "{}", s_orient)
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sdir = match *self {
            Dir::Plus  => "+",
            Dir::Minus => "-"
        };
        write!(f, "{}", sdir)
    }
}





macro_rules! rotate_edge {
    ( $result:expr, $sides:expr, $dir:expr, $side:ident, (Dir::$dir1:ident =>  $side_rotation1:ident), (Dir::$dir2:ident =>  $side_rotation2:ident) )  => (
        $result.$side =
            match $dir {
                Dir::$dir1  =>
                        side::for_cube::rotation(&$sides.$side,  side::for_cube::Dir::$side_rotation1),
                Dir::$dir2  =>
                        side::for_cube::rotation(&$sides.$side,  side::for_cube::Dir::$side_rotation2),
            };
    )
}

fn horizontal(sides: &Sides, dir: Dir, level: usize) -> Sides
{
    let mut result = *sides;

    macro_rules! get_row {
        ($side:ident, $level:expr)  =>  (sides.$side.row($level))
    }

    macro_rules! switch_rows {
        ($ssource:ident, $sdest:ident)  =>
            (result.$sdest  = side::for_cube::merge_row(&sides.$sdest,  level, &get_row!($ssource, level));)
    }

    macro_rules! rotate_edge_local {
            ( $side:ident, (Dir::$dir1:ident =>  $side_rotation1:ident), (Dir::$dir2:ident =>  $side_rotation2:ident) )  =>
                (rotate_edge!(result, sides, dir, $side, (Dir::$dir1 => $side_rotation1), (Dir::$dir2 => $side_rotation2)))
    }

    match dir {
        Dir::Plus  => {
            switch_rows!(back,  left);
            switch_rows!(left,  front);
            switch_rows!(front, right);
            switch_rows!(right, back);
        },
        Dir::Minus => {
            switch_rows!(front, left);
            switch_rows!(back,  right);
            switch_rows!(right, front);
            switch_rows!(left,  back);
        }
    };
    match level+1 {
        1               =>     rotate_edge_local!(top,    (Dir::Plus => InvClock), (Dir::Minus => Clock)),
        config::SIZE    =>     rotate_edge_local!(bottom, (Dir::Plus => Clock),    (Dir::Minus => InvClock)),
        _ => ()
    }
    return result;
}

fn vertical(sides: &Sides, dir: Dir, level: usize) -> Sides
{
    let mut result = *sides;

    macro_rules! get_col {
        ($side:ident, $level:expr)  =>  (sides.$side.col($level))
    }

    macro_rules! rotate_edge_local {
            ( $side:ident, (Dir::$dir1:ident =>  $side_rotation1:ident), (Dir::$dir2:ident =>  $side_rotation2:ident) )  =>
                (rotate_edge!(result, sides, dir, $side, (Dir::$dir1 => $side_rotation1), (Dir::$dir2 => $side_rotation2)))
    }

    macro_rules! switch_cols {
        ($ssource:ident, $sdest:ident)  =>
            (result.$sdest  = side::for_cube::merge_col(&sides.$sdest,  level, &get_col!($ssource, level));)
    }

    match dir {
        Dir::Plus  => {
            switch_cols!(top,    front);
            switch_cols!(front,  bottom);
            switch_cols!(bottom, back);
            switch_cols!(back,   top);
        },
        Dir::Minus => {
            switch_cols!(top,    back);
            switch_cols!(back,   bottom);
            switch_cols!(bottom, front);
            switch_cols!(front,  top);
        }
    };
    match level+1 {
        1               =>     rotate_edge_local!(left,    (Dir::Plus => Clock),    (Dir::Minus => InvClock)),
        config::SIZE    =>     rotate_edge_local!(right,   (Dir::Plus => InvClock), (Dir::Minus => Clock)),
        _ => ()
    }
    return result;
}

fn front(sides: &Sides, dir: Dir, level: usize) -> Sides
{
    let mut result = *sides;

    macro_rules! get_col {
        ($side:ident, $level:expr)  =>  (sides.$side.col($level))
    }
    macro_rules! get_row {
        ($side:ident, $level:expr)  =>  (sides.$side.row($level))
    }

    macro_rules! rotate_edge_local {
            ( $side:ident, (Dir::$dir1:ident =>  $side_rotation1:ident), (Dir::$dir2:ident =>  $side_rotation2:ident) )  =>
                (rotate_edge!(result, sides, dir, $side, (Dir::$dir1 => $side_rotation1), (Dir::$dir2 => $side_rotation2)))
    }

    macro_rules! switch_col2row {
        (($ssource:ident, $slevel:expr) =>  ($sdest:ident, $dlevel:expr))  =>
            (result.$sdest  = side::for_cube::merge_row(&sides.$sdest,  $dlevel, &get_col!($ssource, $slevel));)
    }
    macro_rules! switch_row2col {
        (($ssource:ident, $slevel:expr)  =>  ($sdest:ident, $dlevel:expr))  =>
            (result.$sdest  = side::for_cube::merge_col(&sides.$sdest,  $dlevel, &get_row!($ssource, $slevel));)
    }

    let invert = | r_c : &[u8; config::SIZE] | {
        let mut result = *r_c;
        result.reverse();
        result
    };
    macro_rules! switch_col2row_invert {
        (($ssource:ident, $slevel:expr) =>  ($sdest:ident, $dlevel:expr))  =>
            (result.$sdest  = side::for_cube::merge_row(&sides.$sdest,  $dlevel, &invert(&get_col!($ssource, $slevel)));)
    }
    macro_rules! switch_row2col_invert {
        (($ssource:ident, $slevel:expr)  =>  ($sdest:ident, $dlevel:expr))  =>
            (result.$sdest  = side::for_cube::merge_col(&sides.$sdest,  $dlevel, &invert(&get_row!($ssource, $slevel)));)
    }

    match dir {
        Dir::Plus  => {
            switch_row2col!        ((top,    config::SIZE - level-1)   =>      (right, level) );
            switch_col2row_invert! ((right,  level)                    =>      (bottom, level));
            switch_row2col!        ((bottom, level)                    =>      (left, config::SIZE - level-1));
            switch_col2row_invert! ((left, config::SIZE - level-1)     =>      (top, config::SIZE - level-1));
        },
        Dir::Minus => {
            switch_row2col_invert! ((top, config::SIZE - level-1)      =>      (left, config::SIZE - level-1));
            switch_col2row!        ((left, config::SIZE - level-1)     =>      (bottom, level));
            switch_row2col_invert! ((bottom, level)                    =>      (right, level));
            switch_col2row!        ((right, level)                     =>      (top, config::SIZE - level-1));
        }
    };
    match level+1 {
        1               =>     rotate_edge_local!(front,   (Dir::Plus => Clock),    (Dir::Minus => InvClock)),
        config::SIZE    =>     rotate_edge_local!(back,    (Dir::Plus => InvClock), (Dir::Minus => Clock)),
        _ => ()
    }
    return result;
}


#[test]
fn test_display_rotation()
{
    {
        let mv = Item(Orient::Horizontal, Dir::Plus, 0);
        assert_eq!(format!("{}", mv), "H+0");
    }
    {
        let mv = Item(Orient::Front, Dir::Minus, 2);
        assert_eq!(format!("{}", mv), "F-2");
    }
    {
        let mv = Item(Orient::Vertical, Dir::Plus, 4);
        assert_eq!(format!("{}", mv), "V+4");
    }
}


#[test]
fn test_rotation() {
    let cube = super::create(
                                &side::color(0),
            &side::color(1),    &side::color(2),    &side::color(3),
                                &side::color(4),
                                &side::color(5),
        );

    let str_cube = concat!(
            "      0000  \n",
            "      0000  \n",
            "      0000  \n",
            "      5555  \n",
            "\n",
            "4555  1111  0222  \n",
            "1111  0222  3333  \n",
            "1111  0222  3333  \n",
            "1111  0222  3333  \n",
            "\n",
            "      2444  \n",
            "      2444  \n",
            "      2444  \n",
            "      2444  \n",
            "\n",
            "      3333  \n",
            "      4555  \n",
            "      4555  \n",
            "      4555  \n",
            "\n");

    let rotation1 = vertical(&cube, Dir::Plus, 0);
    let rotation2 = horizontal(&rotation1, Dir::Plus, 0);

    assert_eq!(str_cube, format!("{}", rotation2));
}

#[test]
fn test_front() {
    let cube = super::create(
                                &side::color(0),
            &side::color(1),    &side::color(2),    &side::color(3),
                                &side::color(4),
                                &side::color(5),
        );

    {
        let str_cube = concat!(
                "      0000  \n",
                "      0000  \n",
                "      0000  \n",
                "      1111  \n",
                "\n",
                "1114  2222  0333  \n",
                "1114  2222  0333  \n",
                "1114  2222  0333  \n",
                "1114  2222  0333  \n",
                "\n",
                "      3333  \n",
                "      4444  \n",
                "      4444  \n",
                "      4444  \n",
                "\n",
                "      5555  \n",
                "      5555  \n",
                "      5555  \n",
                "      5555  \n",
                "\n");
        let rotation1 = front(&cube, Dir::Plus, 0);
        assert_eq!(str_cube, format!("{}", rotation1));
    }
    {
        let str_cube = concat!(
                "      0000  \n",
                "      3333  \n",
                "      0000  \n",
                "      0000  \n",
                "\n",
                "1011  2222  3343  \n",
                "1011  2222  3343  \n",
                "1011  2222  3343  \n",
                "1011  2222  3343  \n",
                "\n",
                "      4444  \n",
                "      4444  \n",
                "      1111  \n",
                "      4444  \n",
                "\n",
                "      5555  \n",
                "      5555  \n",
                "      5555  \n",
                "      5555  \n",
                "\n");
        let rotation1 = front(&cube, Dir::Minus, 2);
        assert_eq!(str_cube, format!("{}", rotation1));
    }
    {
        let str_cube = concat!(
                "      5000  \n",
                "      5000  \n",
                "      5000  \n",
                "      1111  \n",
                "\n",
                "1112  0000  5333  \n",
                "1114  2222  0333  \n",
                "1114  2222  0333  \n",
                "1114  2222  0333  \n",
                "\n",
                "      3333  \n",
                "      2444  \n",
                "      2444  \n",
                "      2444  \n",
                "\n",
                "      4555  \n",
                "      4555  \n",
                "      4555  \n",
                "      4555  \n",
                "\n");
        let rotation1 = vertical(&cube, Dir::Plus, 0);
        let rotation2 = front(&rotation1, Dir::Plus, 0);
        assert_eq!(str_cube, format!("{}", rotation2));
    }
}


#[test]
fn test_rotation_front2() {
    let init : super::Sides = super::create_from_strings(
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

    let end : super::Sides = super::create_from_strings(
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

    {
        let rotation = front(&init, Dir::Plus, 0);
        assert_eq!(rotation, end);
    }

    {
        use super::rot;
        use super::rot::{Orient, Dir};

        assert_eq!(init
                    .get_rotation(&rot::Item(Orient::Front, Dir::Plus, 0))
                    .get_rotation(&rot::Item(Orient::Front, Dir::Plus, 0))
                    .get_rotation(&rot::Item(Orient::Front, Dir::Plus, 0))
                    .get_rotation(&rot::Item(Orient::Front, Dir::Plus, 0))
            , init);
    }
}
