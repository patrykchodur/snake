use color_eyre::eyre::OptionExt;
/// Application result type.
pub use color_eyre::Result as AppResult;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum SnakeDirection {
    Left,
    Right,
    Up,
    Down,
}

impl SnakeDirection {
    fn is_opposite(&self, other: &SnakeDirection) -> bool {
        match (self, other) {
            (SnakeDirection::Left, SnakeDirection::Right)
            | (SnakeDirection::Right, SnakeDirection::Left) => true,
            (SnakeDirection::Up, SnakeDirection::Down)
            | (SnakeDirection::Down, SnakeDirection::Up) => true,
            _ => false,
        }
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Does the snake wraps around the edges?
    pub wrap: bool,
    /// Is the snake alive?
    pub is_alive: bool,
    /// The number of points in the map
    pub map_size: (isize, isize),
    /// The direction in which the snake is moving
    pub snake_direction: SnakeDirection,
    /// Next direction of the snake.
    /// We store it here to change it only during ticks
    next_direction: SnakeDirection,
    /// The vector holding existing points of the snake
    /// The boolean indicates whether it's an eaten fruit
    pub snake_points: Vec<(isize, isize, bool)>,
    /// Location of the new fruit
    pub uneaten_fruit: Option<(isize, isize)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            wrap: true,
            is_alive: true,
            map_size: (50, 50),
            snake_direction: SnakeDirection::Left,
            next_direction: SnakeDirection::Left,
            snake_points: vec![
                (25, 25, false),
                (26, 25, false),
                /*
                (27, 25, false),
                (28, 25, false),
                (29, 25, false),
                (30, 25, false),
                (31, 25, false),
                (32, 25, false),
                (33, 25, false),
                */
            ],
            uneaten_fruit: Some((10, 10)),
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
        if !self.is_alive {
            return Ok(());
        }

        self.snake_direction = self.next_direction;

        let next_head = self.get_next_snake_head()?;

        // Get the current back of the snake, which we plan to remove
        let current_back = self
            .snake_points
            .last_mut()
            .ok_or_eyre("There's no last element of snake")?;

        // If the current back is a pixel with a fruit we don't remove it
        if current_back.2 {
            current_back.2 = false;
        } else {
            self.snake_points.pop().ok_or_eyre("Some internal error")?;
        }

        // Check if the snake is eating itself
        for point in &self.snake_points {
            if next_head.0 == point.0 && next_head.1 == point.1 {
                self.is_alive = false;
            }
        }

        // Check if we're out of bounds
        if !self.wrap {
            if next_head.0 < 0
                || next_head.0 >= self.map_size.0
                || next_head.1 < 0
                || next_head.1 >= self.map_size.1
            {
                self.is_alive = false;
            }
        }

        // Add new point to the snake vector
        let new_head_is_a_fruit = self.uneaten_fruit == Some(next_head);
        self.snake_points
            .insert(0, (next_head.0, next_head.1, new_head_is_a_fruit));
        if new_head_is_a_fruit {
            self.spawn_new_fruit()?;
        }

        Ok(())
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) -> AppResult<()> {
        self.running = false;
        Ok(())
    }

    /// Change the direction of the snake from the event handler
    pub fn change_direction(&mut self, new_direction: SnakeDirection) -> AppResult<()> {
        // We don't change the actual direction here. It's done in the `tick` function instead
        if !self.snake_direction.is_opposite(&new_direction) {
            self.next_direction = new_direction;
        }
        Ok(())
    }

    /// Set the dimentions of the screen
    pub fn set_screen_size(&mut self, (width, height): (usize, usize)) -> AppResult<()> {
        self.map_size = (width as isize, height as isize);
        Ok(())
    }

    fn spawn_new_fruit(&mut self) -> AppResult<()> {
        // Generate a random point that is not in the snake
        let mut rng = rand::thread_rng();
        let snake_points = &self.snake_points;
        let (map_width, map_height) = self.map_size;

        // This is dumb implementation
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(0..map_width);
            y = rng.gen_range(0..map_height);
            if !snake_points
                .iter()
                .any(|snake_point| snake_point.0 == x && snake_point.1 == y)
            {
                break;
            }
        }

        self.uneaten_fruit = Some((x, y));
        Ok(())
    }

    fn get_next_snake_head(&self) -> AppResult<(isize, isize)> {
        // Get the current head of snake
        let current_head = self
            .snake_points
            .first()
            .ok_or_eyre("The snake is empty!")?;

        // Calculate next point of snake
        let mut next_head = match self.snake_direction {
            SnakeDirection::Left => (current_head.0 - 1, current_head.1),
            SnakeDirection::Right => (current_head.0 + 1, current_head.1),
            SnakeDirection::Up => (current_head.0, current_head.1 + 1),
            SnakeDirection::Down => (current_head.0, current_head.1 - 1),
        };

        // Wrap snake around the edges
        if self.wrap {
            next_head.0 = next_head.0.rem_euclid(self.map_size.0);
            next_head.1 = next_head.1.rem_euclid(self.map_size.1);
        }

        Ok(next_head)
    }
}
