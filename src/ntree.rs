

pub trait Point {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointThing {
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub r:f32,
    pub g:f32,
    pub b:f32,
    pub a:f32,
}
impl PointThing {
    pub fn new(x:f32, y:f32) -> Self {
        Self { x, y, z:0., r:1., g:1., b:1., a:1. }
    }
    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z, self.r, self.g, self.b, self.a]
    }
}
impl Point for PointThing {
    fn get_x(&self) -> f32 { self.x }
    fn get_y(&self) -> f32 { self.y }
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
    pub fn middle(&self) -> PointThing {
        PointThing::new((self.xl+self.xh)/2., (self.yl+self.yh)/2.)
    }
}

#[derive(Debug)]
pub enum TreeError {
    CannotSubdivideNode,
    CannotSubdivideValue,
    NodeIsNone,
    PointOutOfBounds,
    NodeAlreadySubdivided,
    QuadTreeNotInitialised,
    SituationNotYetEncounteredOrTested,
}


#[derive(Debug)]
pub enum TreeIndex {
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuadTree<T> {
    pub top_left     : QuadTreeItem<T>,
    pub top_right    : QuadTreeItem<T>,
    pub bottom_left  : QuadTreeItem<T>,
    pub bottom_right : QuadTreeItem<T>,
    pub out_of_bounds_points:Vec<T>,
    pub bounds       : SquareBounds,
}
#[derive(Debug, Clone, PartialEq)]
pub enum QuadTreeItem<T> {
    Node(Box<QuadTree<T>>),
    Value(Vec<T>),
    None
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelativePointPos {
    OutOfBounds,

    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl<T:Clone+Point> QuadTree<T> {
    pub fn get_by(self, relative_position:RelativePointPos) -> Result<QuadTreeItem<T>, TreeError> {
        match relative_position {
            RelativePointPos::OutOfBounds => Err(TreeError::PointOutOfBounds),
            RelativePointPos::TopLeft     => Ok(self.top_left),
            RelativePointPos::TopRight    => Ok(self.top_right),
            RelativePointPos::BottomLeft  => Ok(self.bottom_left),
            RelativePointPos::BottomRight => Ok(self.bottom_right),
        }
    }
    pub fn new(bounds:SquareBounds) -> Self {
        Self {
            top_left: QuadTreeItem::None,
            top_right: QuadTreeItem::None,
            bottom_left: QuadTreeItem::None,
            bottom_right: QuadTreeItem::None,
            out_of_bounds_points:vec![],
            bounds
        }
    }
    pub fn get_relative_point_pos(&self, point:&T) -> RelativePointPos {
        let (px, py) = (point.get_x(), point.get_y());
        let (xl, xh, yl, yh) = (self.bounds.xl, self.bounds.xh, self.bounds.yl, self.bounds.yh);
        let middle = self.bounds.middle();
        let (xm, ym) = (middle.get_x(), middle.get_y());

        let (left, right) = (xm >= px, xm <= px);
        let (bottom, top) = (ym >= py, ym <= py);
        let (out_left, out_right) = (xl >= px, xh <= px);
        let (out_bottom, out_top) = (yl >= py, yh <= py);

        let top_left = top && left;
        let top_right = top && right;
        let bottom_left = bottom && left;
        let bottom_right = bottom && right;

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
    pub fn insert(mut self, points:Vec<T>, max_per_node:usize) -> Result<Self, TreeError> {
        let mut top_left_points = vec![];
        let mut top_right_points = vec![];
        let mut bottom_left_points = vec![];
        let mut bottom_right_points = vec![];
        let mut out_of_bounds_points = vec![];
        for point in points {
            match self.get_relative_point_pos(&point) {
                RelativePointPos::OutOfBounds => out_of_bounds_points.push(point),
                RelativePointPos::TopLeft => top_left_points.push(point),
                RelativePointPos::TopRight => top_right_points.push(point),
                RelativePointPos::BottomLeft => bottom_left_points.push(point),
                RelativePointPos::BottomRight => bottom_right_points.push(point),
            }
        }
        let corners_initial = [
            (self.top_left, top_left_points, RelativePointPos::TopLeft),
            (self.top_right, top_right_points, RelativePointPos::TopRight),
            (self.bottom_left, bottom_left_points, RelativePointPos::BottomLeft),
            (self.bottom_right, bottom_right_points, RelativePointPos::BottomRight),
        ];

        let mut resulting_quadtrees = [QuadTreeItem::None, QuadTreeItem::None, QuadTreeItem::None, QuadTreeItem::None];
        for (idx, (mut corner, points, relative_position)) in corners_initial.into_iter().enumerate() {
            let quad_tree_i = match corner {
                QuadTreeItem::None => {
                    //corner = QuadTreeItem::Value(points); corner
                    if points.len() >= max_per_node {
                        let mut here_tree = QuadTree::new(self.bounds).subdivide_node(relative_position, max_per_node)?;
                        let h2 = here_tree.insert(points, max_per_node)?;
                        let c = h2.get_by(relative_position)?;
                        c
                    }
                    else {
                        corner = QuadTreeItem::Value(points);
                        corner
                    }
                },
                QuadTreeItem::Value(mut vec) => {
                    vec.extend(points);
                    if vec.len() >= 3 {
                        Err(TreeError::SituationNotYetEncounteredOrTested)?;

                        // a guess of how to deal with this scenario
                        let mut here_tree = QuadTree::new(self.bounds).subdivide_node(relative_position, max_per_node)?;
                        let h2 = here_tree.insert(vec, max_per_node)?;
                        let c = QuadTreeItem::Node(Box::new(h2));
                        c
                    }
                    else {
                        corner = QuadTreeItem::Value(vec);
                        corner
                    }
                },
                QuadTreeItem::Node( mut node) => {
                    let new_node = node.insert(points, max_per_node)?;
                    QuadTreeItem::Node(Box::new(new_node))
                },
            };
            resulting_quadtrees[idx] = quad_tree_i;
        }

        Ok(QuadTree {top_left:resulting_quadtrees[0].clone(), top_right:resulting_quadtrees[1].clone(), bottom_left:resulting_quadtrees[2].clone(), bottom_right:resulting_quadtrees[3].clone(), out_of_bounds_points:out_of_bounds_points, bounds:self.bounds})
    }
    //pub fn subdivide_by_num_values(mut self, value_limit:u8) -> Result<Self, TreeError> {
    //    let mut top_left_points = vec![];
    //    let mut top_right_points = vec![];
    //    let mut bottom_left_points = vec![];
    //    let mut bottom_right_points = vec![];
    //    let mut out_of_bounds_points = vec![];
    //    for point in points {
    //        match self.get_relative_point_pos(&point) {
    //            RelativePointPos::OutOfBounds => out_of_bounds_points.push(point),
    //            RelativePointPos::TopLeft => top_left_points.push(point),
    //            RelativePointPos::TopRight => top_right_points.push(point),
    //            RelativePointPos::BottomLeft => bottom_left_points.push(point),
    //            RelativePointPos::BottomRight => bottom_right_points.push(point),
    //        }
    //    }
    //    let corners_initial = [
    //        (self.top_left, top_left_points),
    //        (self.top_right, top_right_points),
    //        (self.bottom_left, bottom_left_points),
    //        (self.bottom_right, bottom_right_points),
    //    ];
//
//
    //    let rq = resulting_quadtrees;
    //    let mut subs = [QuadTreeItem::None, QuadTreeItem::None, QuadTreeItem::None, QuadTreeItem::None];
    //    let corners =  [(self.top_left, RelativePointPos::TopLeft), (self.top_right, RelativePointPos::TopRight), (self.bottom_left, RelativePointPos::BottomLeft), (self.bottom_right, RelativePointPos::BottomRight)];
    //    for (idx, (mut sub_quadtree, relative_position)) in corners.into_iter().enumerate() {
    //        let new_quad_tree_i = match sub_quadtree {
    //            QuadTreeItem::None => sub_quadtree,
    //            QuadTreeItem::Value(vec) => {
    //                if vec.len() >= 4 {
    //                    let mut q2 = QuadTree::new(self.bounds).subdivide_node(relative_position)?;
    //                    let q3 = q2.insert(vec)?;
    //                    
    //                }
    //            },
    //            QuadTreeItem::Node(node) => {},
    //        }
    //        
    //        
    //        if let QuadTreeItem::Value(vec) = sub_quadtree {
    //            if vec.len() >= 4 {
    //                let q = QuadTree::new(self.bounds).insert(vec)?.subdivide_node(relative_position)?;
    //                sub_quadtree = QuadTreeItem::Node(Box::new(q));
    //                subs[idx] = sub_quadtree;
    //            }
    //        }
    //    }
//
    //    Ok(QuadTree {top_left:resulting_quadtrees[0].clone(), top_right:resulting_quadtrees[1].clone(), bottom_left:resulting_quadtrees[2].clone(), bottom_right:resulting_quadtrees[3].clone(), out_of_bounds_points:out_of_bounds_points, bounds:self.bounds})
    //    //Ok(QuadTree {top_left:resulting_quadtrees[0].clone(), top_right:resulting_quadtrees[1].clone(), bottom_left:resulting_quadtrees[2].clone(), bottom_right:resulting_quadtrees[3].clone(), out_of_bounds_points:out_of_bounds_points, bounds:self.bounds})
    //    //Ok(QuadTree {top_left:qt1, top_right:qt2, bottom_left:qt3, bottom_right:qt4, out_of_bounds_points:out_of_bounds_points, bounds:self.bounds})
    //}
    fn _subdivide_node_internal_match(
        &self, current_node:QuadTreeItem<T>, new_bounds:SquareBounds, max_per_node:usize
    ) -> Result<QuadTreeItem<T>, TreeError> {

        let mut new_quadtree = QuadTree::new(new_bounds);
        let new_new_quadtree = match current_node {
            QuadTreeItem::None => new_quadtree,
            QuadTreeItem::Node(node) => {Err(TreeError::NodeAlreadySubdivided)?; new_quadtree},
            QuadTreeItem::Value(points) => { new_quadtree.clone().insert(points, max_per_node)? },
        };
        let mut new_current_node = QuadTreeItem::Node(Box::new(new_new_quadtree));
        Ok(new_current_node)
    }
    pub fn subdivide_node(mut self, quarter:RelativePointPos, max_per_node:usize) -> Result<Self, TreeError> {
        let current_middle = self.bounds.middle();
        match quarter {
            RelativePointPos::OutOfBounds => Err(TreeError::PointOutOfBounds)?,
            RelativePointPos::TopLeft => {
                let (new_low_x, new_high_x) = (self.bounds.xl, current_middle.x);
                let (new_low_y, new_high_y) = (current_middle.y, self.bounds.yh);
                let new_bounds = SquareBounds::new(new_low_x, new_high_x, new_low_y, new_high_y);
                self.top_left = self._subdivide_node_internal_match(self.top_left.clone(), new_bounds, max_per_node)?;
            },
            RelativePointPos::TopRight => {
                let (new_low_x, new_high_x) = (current_middle.x, self.bounds.xh);
                let (new_low_y, new_high_y) = (current_middle.y, self.bounds.yh);
                let new_bounds = SquareBounds::new(new_low_x, new_high_x, new_low_y, new_high_y);
                self.top_right = self._subdivide_node_internal_match(self.top_right.clone(), new_bounds, max_per_node)?;

            },
            RelativePointPos::BottomLeft => {
                let (new_low_x, new_high_x) = (self.bounds.xl, current_middle.x);
                let (new_low_y, new_high_y) = (self.bounds.yl, current_middle.y);
                let new_bounds = SquareBounds::new(new_low_x, new_high_x, new_low_y, new_high_y);
                self.bottom_left = self._subdivide_node_internal_match(self.bottom_left.clone(), new_bounds, max_per_node)?;
            },
            RelativePointPos::BottomRight => {
                let (new_low_x, new_high_x) = (current_middle.x, self.bounds.xh);
                let (new_low_y, new_high_y) = (self.bounds.yl, current_middle.y);
                let new_bounds = SquareBounds::new(new_low_x, new_high_x, new_low_y, new_high_y);
                self.bottom_right = self._subdivide_node_internal_match(self.bottom_right.clone(), new_bounds, max_per_node)?;

            },
        }
        Ok(self)
    }
    pub fn get_all_values(&self) -> Vec<T> {
        let mut all_values = vec![];
        if let QuadTreeItem::Value(val) = &self.top_left {
            all_values.extend((*val).clone());
        }
        if let QuadTreeItem::Value(val) = &self.top_right {
            all_values.extend((*val).clone());
        }
        if let QuadTreeItem::Value(val) = &self.bottom_left {
            all_values.extend((*val).clone());
        }
        if let QuadTreeItem::Value(val) = &self.bottom_right {
            all_values.extend((*val).clone());
        }

        if let QuadTreeItem::Node(node) = &self.top_left {
            all_values.extend(node.get_all_values());
        }
        if let QuadTreeItem::Node(node) = &self.top_right {
            all_values.extend(node.get_all_values());
        }
        if let QuadTreeItem::Node(node) = &self.bottom_left {
            all_values.extend(node.get_all_values());
        }
        if let QuadTreeItem::Node(node) = &self.bottom_right {
            all_values.extend(node.get_all_values());
        }
        all_values.extend(self.out_of_bounds_points.clone());
        all_values
    }
    pub fn get_all_bounds(&self) -> Vec<SquareBounds> {
        let mut all_bounds = vec![];
        all_bounds.push(self.bounds);
        if let QuadTreeItem::Node(node) = &self.top_left {
            all_bounds.extend(node.get_all_bounds());
        }
        if let QuadTreeItem::Node(node) = &self.top_right {
            all_bounds.extend(node.get_all_bounds());
        }
        if let QuadTreeItem::Node(node) = &self.bottom_left {
            all_bounds.extend(node.get_all_bounds());
        }
        if let QuadTreeItem::Node(node) = &self.bottom_right {
            all_bounds.extend(node.get_all_bounds());
        }
        all_bounds
    }
    pub fn get_all_lines(&self) -> Vec<f32> {
        let all_bounds = self.get_all_bounds();
        let mut all_lines_data = vec![];
        for bounds in all_bounds {
            let (xl, xh, yl, yh) = (bounds.xl, bounds.xh, bounds.yl, bounds.yh);
            all_lines_data.extend(vec![
                xl, yl, 0., 1., 1., 1., 1., // bottom line
                xh, yl, 0., 1., 1., 1., 1., // bottom line

                xl, yh, 0., 1., 1., 1., 1., // top line
                xh, yh, 0., 1., 1., 1., 1., // top line

                xl, yl, 0., 1., 1., 1., 1., // left line
                xl, yh, 0., 1., 1., 1., 1., // left line

                xh, yl, 0., 1., 1., 1., 1., // right line
                xh, yh, 0., 1., 1., 1., 1., // right line
            ]);
        }
        all_lines_data
    }
}
impl QuadTree<PointThing> {
    pub fn get_all_points(&self) -> Vec<f32> {
        let all_points = self.get_all_values();
        let mut all_points_data = vec![];
        for point in all_points {
            all_points_data.extend(point.to_vec());
        }
        all_points_data
    }
}