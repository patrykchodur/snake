use color_eyre::eyre::OptionExt;
/// Application result type.
pub use color_eyre::Result as AppResult;

#[derive(Debug)]
pub enum SnakeDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Is the snake alive?
    pub is_alive: bool,
    /// The number of points in the map
    pub map_size: (usize, usize),
    /// The direction in which the snake is moving
    pub snake_direction: SnakeDirection,
    /// The vector holding existing points of the snake
    pub snake_points: Vec<(usize, usize)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            is_alive: true,
            map_size: (50, 50),
            snake_direction: SnakeDirection::Left,
            snake_points: vec![(25, 25), (26, 25)],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) -> AppResult<()> {
        let current_head = self
            .snake_points
            .first()
            .ok_or_eyre("The snake is empty!")?;

        // Calculate next point of snake
        let next_head = match self.snake_direction {
            SnakeDirection::Left => (current_head.0 - 1, current_head.1),
            SnakeDirection::Right => (current_head.0 + 1, current_head.1),
            SnakeDirection::Up => (current_head.0, current_head.1 + 1),
            SnakeDirection::Down => (current_head.0, current_head.1 - 1),
        };
        // Check if the snake is eating itself
        for point in &self.snake_points {
            if next_head == *point {
                self.is_alive = false;
            }
        }
        // Add new point to the snake vector
        self.snake_points.insert(0, next_head);
        self.snake_points.pop().ok_or_eyre("Some internal error")?;

        Ok(())
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) -> AppResult<()> {
        self.running = false;
        Ok(())
    }

    pub fn change_direction(&mut self, new_direction: SnakeDirection) -> AppResult<()> {
        self.snake_direction = new_direction;
        Ok(())
    }
}
