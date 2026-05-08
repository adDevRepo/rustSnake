// importation du module prelude de macroquad
use macroquad::prelude::*;

// importation de l'enum direction
use crate::direction::Direction;


// génération de la position
#[derive(Clone, Copy)]
pub struct Position {

    // entier 32 bits
    x: i32,
    y: i32,
}

//génération du snake
pub struct Snake {
    pub body: Vec<Position>,
    pub direction: Direction,
}


// implémentation des méthodes
impl Snake {
    // Constructeur
    pub fn new() -> Self {
        Self {
            body: vec![
                //tête du snake
                Position {x:5, y:5},

                //corps du snake
                Position {x: 4, y:5},

                //queue
                Position {x: 3, y:5},


            ],

            //direction initiale

            direction: Direction::Right,
        }
    }

    // Déplacement
    pub fn move_forward(&mut self) {

        let mut new_head = self.body[0];

        //changement des coordonnées en fonction de la direction
        match self.direction {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        self.body.insert(0, new_head);

        self.body.pop();
    }

    // visuel
    pub fn visual(&self) {
        const CELL_SIZE: f32 = 20.0;

        for part in &self.body {

            //dessin du rectangle
            draw_rectangle(
                //position x
                part.x as f32 * CELL_SIZE,
                //position y
                part.y as f32 * CELL_SIZE,
                //largeur
                CELL_SIZE,
                //hauteur
                CELL_SIZE,
                GREEN,
            )

        }

    }

}


