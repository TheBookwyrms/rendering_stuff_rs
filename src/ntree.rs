use std::{fmt::{Debug, Display}, ops::Deref};

#[derive(Debug)]
pub enum TreeError {
    CannotSubdivideNode,
    CannotSubdivideValue,
    NodeIsNone,
    PointOutOfBounds,
    NodeAlreadySubdivided,
    QuadTreeNotInitialised,
}


#[derive(Debug)]
pub enum TreeIndex {
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuadTreeV2 {
    pub top_left     : QuadTreeItemV2,
    pub top_right    : QuadTreeItemV2,
    pub bottom_left  : QuadTreeItemV2,
    pub bottom_right : QuadTreeItemV2,
    pub bounds       : SquareBounds,
}
#[derive(Debug, Clone, PartialEq)]
pub enum QuadTreeItemV2 {
    Node(Box<QuadTreeV2>),
    Value(Vec<Point>),
    None
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelativePointPos {
    OutOfBounds,

    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,

    //Top,
    //Right,
    //Bottom,
    //Left,
//
    //Centre,
}

impl QuadTreeV2 {
    pub fn new(bounds:SquareBounds) -> Self {
        Self {
            top_left: QuadTreeItemV2::None,
            top_right: QuadTreeItemV2::None,
            bottom_left: QuadTreeItemV2::None,
            bottom_right: QuadTreeItemV2::None,
            bounds
        }
    }
    pub fn get_relative_point_pos(&self, point:Point) -> RelativePointPos {
        let (px, py) = (point.x, point.y);
        let (xl, xh, yl, yh) = (self.bounds.xl, self.bounds.xh, self.bounds.yl, self.bounds.yh);
        let middle = self.bounds.middle();
        let (xm, ym) = (middle.x, middle.y);

        let (left, right) = (xm >= px, xm <= px);
        let (bottom, top) = (ym >= py, ym <= py);
        let (out_left, out_right) = (xl >= px, xh <= px);
        let (out_bottom, out_top) = (yl >= py, yh <= py);

        let top_left = top && left;
        let top_right = top && right;
        let bottom_left = bottom && left;
        let bottom_right = bottom && right;

        let x = -1;

        if out_bottom || out_left || out_right || out_top {
            RelativePointPos::OutOfBounds
        } else if left {
            if right { // centre x
                if top {
                    if bottom { // centre y
                        // centre gets reduced to top right
                        RelativePointPos::TopRight
                    } else { // top
                        // top centre gets reduced to top right
                        RelativePointPos::TopRight
                    }
                } else { // bottom
                    // bottom centre gets reduced to bottom left
                    RelativePointPos::BottomLeft
                }
            } else { // on left
                if top {
                    if bottom { // centre y
                        // left centre gets reduced to top left
                        RelativePointPos::TopLeft
                    } else { // top
                        RelativePointPos::TopLeft
                    }
                } else { // bottom
                    RelativePointPos::BottomLeft
                }

            }
        } else { // right
            if top {
                if bottom { // centre y
                    // right centre gets reduced to bottom right
                    RelativePointPos::BottomRight
                } else { // top
                    RelativePointPos::TopRight
                }
            } else { // bottom
                RelativePointPos::BottomRight
            }
        }
    }
    pub fn insert(mut self, points:Vec<Point>) -> Result<Self, TreeError> {
        let mut top_left_points = vec![];
        let mut top_right_points = vec![];
        let mut bottom_left_points = vec![];
        let mut bottom_right_points = vec![];
        for point in points {
            match self.get_relative_point_pos(point) {
                RelativePointPos::OutOfBounds => Err(TreeError::PointOutOfBounds)?,
                RelativePointPos::TopLeft => top_left_points.push(point),
                RelativePointPos::TopRight => top_right_points.push(point),
                RelativePointPos::BottomLeft => bottom_left_points.push(point),
                RelativePointPos::BottomRight => bottom_right_points.push(point),
            }
        }

        let mut quadtree = self.clone();
        let qt1 = match self.top_left {
            QuadTreeItemV2::None => {self.top_left = QuadTreeItemV2::Value(top_left_points); self.top_left},
            QuadTreeItemV2::Value(mut vec) => {vec.extend(top_left_points); QuadTreeItemV2::Value(vec)},
            QuadTreeItemV2::Node( mut node) => {
                let new_node = node.insert(top_left_points)?;
                QuadTreeItemV2::Node(Box::new(new_node))
            },
        };
        let qt2 = match self.top_right {
            QuadTreeItemV2::None => {self.top_right = QuadTreeItemV2::Value(top_right_points); self.top_right},
            QuadTreeItemV2::Value(mut vec) => {vec.extend(top_right_points); QuadTreeItemV2::Value(vec)},
            QuadTreeItemV2::Node( mut node) => {
                let new_node = node.insert(top_right_points)?;
                QuadTreeItemV2::Node(Box::new(new_node))
            },
            //QuadTreeItemV2::None => self.top_right = QuadTreeItemV2::Value(top_right_points),
            //QuadTreeItemV2::Value(mut vec) => vec.extend(top_right_points),
            //QuadTreeItemV2::Node( mut node) => {node.insert(top_right_points)?;},
        };
        let qt3 = match self.bottom_left {
            QuadTreeItemV2::None => {self.bottom_left = QuadTreeItemV2::Value(bottom_left_points); self.bottom_left},
            QuadTreeItemV2::Value(mut vec) => {vec.extend(bottom_left_points); QuadTreeItemV2::Value(vec)},
            QuadTreeItemV2::Node( mut node) => {
                let new_node = node.insert(bottom_left_points)?;
                QuadTreeItemV2::Node(Box::new(new_node))
            }
            //QuadTreeItemV2::None => self.bottom_left = QuadTreeItemV2::Value(bottom_left_points),
            //QuadTreeItemV2::Value(mut vec) => vec.extend(bottom_left_points),
            //QuadTreeItemV2::Node( mut node) => {node.insert(bottom_left_points)?;},
        };
        let qt4 = match self.bottom_right {
            QuadTreeItemV2::None => {self.bottom_right = QuadTreeItemV2::Value(bottom_right_points); self.bottom_right},
            QuadTreeItemV2::Value(mut vec) => {vec.extend(bottom_right_points); QuadTreeItemV2::Value(vec)},
            QuadTreeItemV2::Node( mut node) => {
                let new_node = node.insert(bottom_right_points)?;
                QuadTreeItemV2::Node(Box::new(new_node))
            }
            //QuadTreeItemV2::None => self.bottom_right = QuadTreeItemV2::Value(bottom_right_points),
            //QuadTreeItemV2::Value(mut vec) => vec.extend(bottom_right_points),
            //QuadTreeItemV2::Node(mut node) => {node.insert(bottom_right_points)?;},
        };
        Ok(QuadTreeV2 {top_left:qt1, top_right:qt2, bottom_left:qt3, bottom_right:qt4, bounds:self.bounds})
        //Ok(self)
    }
    fn _subdivide_node_internal_match(
        &self, current_node:QuadTreeItemV2, new_bounds:SquareBounds
    ) -> Result<QuadTreeItemV2, TreeError> {

        let mut new_quadtree = QuadTreeV2::new(new_bounds);
        //let mut new_quadtree = Box::new(QuadTreeV2::new(new_bounds));
        //let mut new_current_node = QuadTreeItemV2::Node(Box::new(new_quadtree));
        let new_new_quadtree = match current_node {
            QuadTreeItemV2::None => new_quadtree,
            QuadTreeItemV2::Node(node) => {Err(TreeError::NodeAlreadySubdivided)?; new_quadtree},
            QuadTreeItemV2::Value(points) => {
                new_quadtree.clone().insert(points)?
                //let return_value = new_quadtree;
                //return_value
            },
        };
        let mut new_current_node = QuadTreeItemV2::Node(Box::new(new_new_quadtree));
        //let mut new_current_node = QuadTreeItemV2::Node(Box::new(new_quadtree));
        //let mut new_current_node = QuadTreeItemV2::Node(new_quadtree);

        Ok(new_current_node)
    }
    pub fn subdivide_node(mut self, quarter:RelativePointPos) -> Result<Self, TreeError> {
        let current_middle = self.bounds.middle();
        match quarter {
            RelativePointPos::OutOfBounds => Err(TreeError::PointOutOfBounds)?,
            RelativePointPos::TopLeft => {
                let (new_low_x, new_high_x) = (self.bounds.xl, current_middle.x);
                let (new_low_y, new_high_y) = (current_middle.y, self.bounds.yh);
                let new_bounds = SquareBounds::new(new_low_x, new_high_x, new_low_y, new_high_y);
                self.top_left = self._subdivide_node_internal_match(self.top_left.clone(), new_bounds)?;
            },
            RelativePointPos::TopRight => {
                let (new_low_x, new_high_x) = (current_middle.x, self.bounds.xh);
                let (new_low_y, new_high_y) = (current_middle.y, self.bounds.yh);
                let new_bounds = SquareBounds::new(new_low_x, new_high_x, new_low_y, new_high_y);
                self.top_right = self._subdivide_node_internal_match(self.top_right.clone(), new_bounds)?;

            },
            RelativePointPos::BottomLeft => {
                let (new_low_x, new_high_x) = (self.bounds.xl, current_middle.x);
                let (new_low_y, new_high_y) = (self.bounds.yl, current_middle.y);
                let new_bounds = SquareBounds::new(new_low_x, new_high_x, new_low_y, new_high_y);
                self.bottom_left = self._subdivide_node_internal_match(self.bottom_left.clone(), new_bounds)?;
            },
            RelativePointPos::BottomRight => {
                let (new_low_x, new_high_x) = (current_middle.x, self.bounds.xh);
                let (new_low_y, new_high_y) = (self.bounds.yl, current_middle.y);
                let new_bounds = SquareBounds::new(new_low_x, new_high_x, new_low_y, new_high_y);
                self.bottom_right = self._subdivide_node_internal_match(self.bottom_right.clone(), new_bounds)?;

            },
        }
        Ok(self)
    }
}










#[derive(Debug, Clone)]
pub enum QuadTree<T:Debug> {
    Node([Box<QuadTree<T>>; 4]),
    Value(T),
}

impl<T:Debug> Display for QuadTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(val) => write!(f, "{:?}", val),
            Self::Node(node) => {
                
                write!(f, "{{")?;
                
                write!(f, "{}, {}, {}, {}", node[0], node[1], node[2], node[3])?;
                write!(f, "}}")
            },
        }
    }
}



impl<T:Debug+Clone> QuadTree<T> {
    pub fn init_value(item:T)-> Self {
        QuadTree::Value(item)
    }
    pub fn subdivide_value(&self, items:[QuadTree<T>; 4]) -> Result<Self, TreeError> {
        match *self {
            Self::Node(_) => Err(TreeError::CannotSubdivideNode),
            Self::Value(_) => {
                let (x, y, z, w) = items.into();
                Ok(Self::Node([
                    Box::new(x),
                    Box::new(y),
                    Box::new(z),
                    Box::new(w)
                    ]))
            },
        }
    }
    pub fn subdivide_item(&mut self, idx:TreeIndex, items:[QuadTree<T>; 4]) -> Result<Self, TreeError> {
        match self.clone() {
            Self::Value(_) => Err(TreeError::CannotSubdivideValue),
            Self::Node(mut node) => {
                //let (q1, q2, q3, q4) = node.into();
                match idx {
                    TreeIndex::One   => {node[0] = Box::new(node[0].subdivide_value(items)?)},
                    TreeIndex::Two   => {node[1] = Box::new(node[1].subdivide_value(items)?)},
                    TreeIndex::Three => {node[2] = Box::new(node[2].subdivide_value(items)?)},
                    TreeIndex::Four  => {node[3] = Box::new(node[3].subdivide_value(items)?)},
                }
                Ok(QuadTree::Node(node))
            },
        }
    }
}









#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x:f32,
    pub y:f32,
}
impl Point {
    pub fn new(x:f32, y:f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SquareBounds {
    pub xl:f32,
    pub xh:f32,
    pub yl:f32,
    pub yh:f32,
}
impl SquareBounds {
    pub fn new_simple_on_origin(low:f32, high:f32) -> Self {
        Self { xl: low, xh: high, yl: low, yh: high }
    }
    pub fn new(xl:f32, xh:f32, yl:f32, yh:f32) -> Self {
        Self { xl, xh, yl, yh }
    }
    pub fn middle(&self) -> Point {
        Point { x: (self.xl+self.xh)/2., y: (self.yl+self.yh)/2. }
    }
}


//pub fn iter_tree<T:Debug+Clone, U, V, X:Fn(U) -> bool, Y:FnMut(V) -> QuadTree<T>>(
//    tree:QuadTree<T>, mut depth:i8, condition:X, map:Y
//) -> QuadTree<T> {
//    if depth == -1 {
//        tree
//    }
//    depth -= 1;
//    match tree {
//        QuadTree::Node(_),
//        QuadTree::Value(_),
//    }
//}