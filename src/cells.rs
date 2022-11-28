use rand::Rng;

pub const BOARD_WIDTH: i16 = 100;
pub const BOARD_HEIGHT: i16 = 70;
pub const BOARD_AREA: i16 = BOARD_WIDTH * BOARD_HEIGHT;


#[derive(Clone)]
pub struct Cell {
    pub is_alive: bool,
    pub id: i16,
}

fn rand_bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.5)
}

pub fn init_vec() -> Vec<Cell> {
    let mut cell_list = vec![];
    for i in 0..=BOARD_AREA {
        let c = Cell {
            is_alive: rand_bool(),
            id: i,
        };
        cell_list.push(c);
    }
    cell_list
}

// can do more with this later(maybe/probably wont)
fn is_cell_neighbor(i: i16) -> bool {
    i >= 0 && i <= BOARD_AREA
}

fn get_neighbor_indexes(cell_id: i16) -> Vec<i16> {
    let neighbors: Vec<i16> = vec![
        // TOP
        cell_id - BOARD_WIDTH - 1,
        cell_id - BOARD_WIDTH,
        cell_id - BOARD_WIDTH + 1,
        // SIDES
        cell_id + 1,
        cell_id - 1,
        // BOTTOM
        cell_id + BOARD_WIDTH - 1,
        cell_id + BOARD_WIDTH,
        cell_id + BOARD_WIDTH + 1,
    ];

    neighbors
        .into_iter()
        .filter(|i| is_cell_neighbor(*i))
        .collect::<Vec<i16>>()
}

fn is_cell_alive(living: bool, neighbor_cells: Vec<Cell>) -> bool {
    let live_count =
        neighbor_cells.into_iter().fold(
            0,
            |total, current| if current.is_alive { total + 1 } else { total },
        );
    match (living, live_count) {
        (true, 2 | 3) => true,
        (false, 3) => true,
        _ => false,
    }
}

pub fn handle_generation_change(curr_cells: Vec<Cell>) -> Vec<Cell> {
    curr_cells
        .clone()
        .into_iter()
        .map(|cell| {
            // todo couldn;t get map to work but should try again now that Ive added Clone to Cell
            let mut neighbors: Vec<Cell> = vec![];
            let neighbors_ids: Vec<i16> = get_neighbor_indexes(cell.id);
            for i in 0..neighbors_ids.len() {
                let n_id = neighbors_ids.get(i).unwrap();
                if let Some(n) = curr_cells.get(*n_id as usize) {
                    neighbors.push(n.clone());
                } else {
                    println!("no cell found at {}", *n_id);
                }
            }
            // todo couldnt get spread to work here for id
            Cell {
                is_alive: is_cell_alive(cell.is_alive, neighbors),
                id: cell.id,
            }
        })
        .collect::<Vec<Cell>>()
}
