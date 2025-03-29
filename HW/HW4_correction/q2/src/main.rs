fn main() {
    let mut grid = initialize_grid();
    
    for generation in 0..24 {
        println!("Generation {}:", generation);
        print_grid(&grid);
        grid = next_generation(&grid);
    }
}

fn initialize_grid() -> Vec<Vec<bool>> {

    let mut grid = Vec::new();
    
    // Add 24 rows
    for _ in 0..24 {
        let row = vec![false; 24];
        grid.push(row);
    }
    
    // initial live cells
    let live_cells = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    for &(x, y) in &live_cells {
        grid[x][y] = true;
    }
    
    grid
}

fn next_generation(current_grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    // Create empty new grid
    let mut new_grid = Vec::new();
    
    // Initialize each row
    for _ in 0..24 {
        let row = vec![false; 24];
        grid.push(row);
    }
    
    // Calculate next state 
    for x in 0..24 {
        for y in 0..24 {
            let current_state = current_grid[x][y];
            let neighbor_count = count_neighbors(current_grid, x, y);
            new_grid[x][y] = compute_new_state(current_state, neighbor_count);
        }
    }
    
    new_grid
}

fn count_neighbors(grid: &[Vec<bool>], x: usize, y: usize) -> u8 {
    let mut count = 0;
    
    // Check all neighbors
    for dx in -1..=1 {
        for dy in -1..=1 {
            // Skip the cell itself
            if dx == 0 && dy == 0 {
                continue;
            }
            
            // Handle grid wrapping with modulo
            let nx = (x as i32 + dx + 24) % 24;
            let ny = (y as i32 + dy + 24) % 24;
            
            // Count if neighbor is alive
            if grid[nx as usize][ny as usize] {
                count += 1;
            }
        }
    }
    
    count
}

fn compute_new_state(current_state: bool, neighbor_count: u8) -> bool {
    // Conway's rules
    if neighbor_count == 2 {
        current_state  // Stay
    } else if neighbor_count == 3 {
        true         
    } else {
        false      
    }
}

fn print_grid(grid: &[Vec<bool>]) {
    for row in grid {
        for &cell in row {
            print!("{}", if cell { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_new_state() {
        
        assert_eq!(compute_new_state(true, 2), true);
        assert_eq!(compute_new_state(true, 3), true);
        
        assert_eq!(compute_new_state(true, 1), false);
        assert_eq!(compute_new_state(true, 4), false);
        
        assert_eq!(compute_new_state(false, 3), true);
        
        assert_eq!(compute_new_state(false, 2), false);
    }

    #[test]
    fn test_count_neighbors() {
        let mut grid = vec![vec![false; 24]; 24];
        grid[23][23] = true;  // Bottom-right corner
        grid[23][0] = true;   // Bottom-left corner
        assert_eq!(count_neighbors(&grid, 0, 0), 2);
        
        let mut grid = vec![vec![false; 24]; 24];
        grid[4][4] = true;
        grid[4][5] = true;
        grid[5][4] = true;
        assert_eq!(count_neighbors(&grid, 5, 5), 3);
    }
}