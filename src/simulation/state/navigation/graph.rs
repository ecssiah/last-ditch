use ultraviolet::IVec3;

pub struct Graph {
    radius: u32,
    size: u32,
    open_vec: Vec<bool>,
    cost_vec: Vec<u8>,
}

impl Graph {
    #[rustfmt::skip]
    pub const NEIGHBOR_OFFSETS: [IVec3; 26] = [
        IVec3::new(-1, -1, -1),
        IVec3::new( 0, -1, -1),
        IVec3::new( 1, -1, -1),
        IVec3::new(-1,  0, -1),
        IVec3::new( 0,  0, -1),
        IVec3::new( 1,  0, -1),
        IVec3::new(-1,  1, -1),
        IVec3::new( 0,  1, -1),
        IVec3::new( 1,  1, -1),
        IVec3::new(-1, -1,  0),
        IVec3::new( 0, -1,  0),
        IVec3::new( 1, -1,  0),
        IVec3::new(-1,  0,  0),
        IVec3::new( 1,  0,  0),
        IVec3::new(-1,  1,  0),
        IVec3::new( 0,  1,  0),
        IVec3::new( 1,  1,  0),
        IVec3::new(-1, -1,  1),
        IVec3::new( 0, -1,  1),
        IVec3::new( 1, -1,  1),
        IVec3::new(-1,  0,  1),
        IVec3::new( 0,  0,  1),
        IVec3::new( 1,  0,  1),
        IVec3::new(-1,  1,  1),
        IVec3::new( 0,  1,  1),
        IVec3::new( 1,  1,  1),
    ];

    pub fn new(radius: u32) -> Self {
        let size = 2 * radius + 1;
        let volume = (size * size * size) as usize;

        let open_vec = vec![false; volume];
        let cost_vec = vec![1u8; volume];

        Self {
            radius,
            size,
            open_vec,
            cost_vec,
        }
    }

    pub fn get_index(position: IVec3, graph: &Graph) -> usize {
        let position_indexable = position + IVec3::broadcast(graph.radius as i32);

        ((position_indexable.z as usize * graph.size as usize + position_indexable.y as usize)
            * graph.size as usize
            + position_indexable.x as usize) as usize
    }

    pub fn position_valid(position: IVec3, graph: &Graph) -> bool {
        let radius = graph.radius as i32;

        position.x >= -radius
            && position.x <= radius
            && position.y >= -radius
            && position.y <= radius
            && position.z >= -radius
            && position.z <= radius
    }

    #[inline]
    pub fn is_open(position: IVec3, graph: &Graph) -> bool {
        Graph::position_valid(position, graph) && graph.open_vec[Self::get_index(position, graph)]
    }

    #[inline]
    pub fn get_cost(position: IVec3, graph: &Graph) -> i32 {
        let index = Self::get_index(position, graph);

        graph.cost_vec[index] as i32
    }

    #[inline]
    pub fn get_valid_neighbor_position_iter(
        position: IVec3,
        graph: &Graph,
    ) -> impl Iterator<Item = IVec3> {
        let mut open_neighbor_position_vec = Vec::with_capacity(Self::NEIGHBOR_OFFSETS.len());

        for neighbor_offset in Self::NEIGHBOR_OFFSETS.iter() {
            let neighbor_position = position + *neighbor_offset;

            if Graph::position_valid(neighbor_position, graph)
                && Graph::is_open(neighbor_position, graph)
            {
                open_neighbor_position_vec.push(neighbor_position);
            }
        }

        open_neighbor_position_vec.into_iter()
    }
}
