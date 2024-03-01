use std::collections::VecDeque;

pub type Position = (usize, usize);

#[derive(Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]
pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: VecDeque<Position>,
    direction: Direction,
    food: Position,
    lost: bool
}

impl SnakeGame {
    pub fn new(width:usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width - 2).max(0), height / 2)].into_iter().collect(),
            direction: Direction::Left,
            food:(2.min(width - 1) , height / 2),
            lost: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Top, Direction::Top) |
            (Direction::Top, Direction::Bottom) |
            (Direction::Right, Direction::Right) |
            (Direction::Right, Direction::Left) |
            (Direction::Bottom, Direction::Top) |          
            (Direction::Bottom, Direction::Bottom) |
            (Direction::Left, Direction::Right) |
            (Direction::Left, Direction::Left) => {}
            (_, direction) => self.direction = direction,
        }
    }

    pub fn is_valid(&self, (x,y): Position) -> bool {
        x < self.width && y < self.height
    }
    pub fn tick(&mut self) {
        if self.lost && self.snake.len() == 0 {
            return
        }
        // Move snake
        let (x, y) = self.snake[0];
        let new_head = match &self.direction {
            Direction::Top => (x, y - 1),
            Direction::Right => (x + 1, y ),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x -1 , y),
        };
       
        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.lost = true
        } else {
            if new_head != self.food{
                self.snake.pop_back();
            } else {
                self.food = todo!();
            }
            
            self.snake.push_front(new_head);
        }
            
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        println!("{:?}", SnakeGame::new(10,10));
        
    }
}
