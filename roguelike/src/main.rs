//Liam R Mitchell, following https://tomassedovic.github.io/roguelike-tutorial/ for fun

///////////////////////////////// consts ////////////////////////////////////////////

//libs 
use tcod::colors::*;
use tcod::console::*;

// actual window size
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// map size, smaller than screen for STATS
const MAP_WIDTH: i32 = 80; 
const MAP_HEIGHT: i32 = 45; 

// fps
const LIMIT_FPS: i32 = 20; // fps max

//colors
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100};
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150};

//////////////////////////////////////////generics and setup /////////////////////////////

struct Tcod { //for doing all libtcod values
	root: Root,
	con: Offscreen,
}

// generic object, rep by character on screen
#[derive(Debug)]
struct Object {
	x: i32,
	y: i32,
	char: char,
	color: Color,
}

impl Object {
	//init object
	pub fn new(x: i32, y: i32, char: char, color: Color) -> Self { 
		Object { x, y, char, color }
	}
	
	//move by given amount
	pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
		if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked { 
            self.x += dx;
		    self.y += dy;
        }
	}

	// set color and draw char that represents obj @ position
	pub fn draw(&self, con: &mut dyn Console) { //dyn means console is trait and not type
		con.set_default_foreground(self.color);
		con.put_char(self.x, self.y, self.char, BackgroundFlag::None); 
	}
}

////////////////////////////////////////map //////////////////////////////////////////

//tile of map and properties
#[derive(Clone, Copy, Debug)] //implement traits for tile
struct Tile {
    blocked: bool, 
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false, 
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true, 
            block_sight: true, 
        }
    }
}

type Map = Vec<Vec<Tile>>; //map is vec of vecs 

struct Game {
    map: Map, //shortcut lets us write Map instead of above:
}

fn make_map() -> Map {
    //fill map with unblocked tiles
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    //vec! fills vec with data (tiles) 

    //test pillars
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();
    
    map
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    //draw all
    for object in objects {
        object.draw(&mut tcod.con);
    }
    //foreach tile set bg color
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    blit(
        &tcod.con, //from
        (0, 0), //start at
        (MAP_WIDTH, MAP_HEIGHT), //size
        &mut tcod.root, //to
        (0, 0),
        1.0,
        1.0,
    );
}

//////////////////////////////////////////////////// handle inputs//////////////////////////////////////////

fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {
	use tcod::input::Key;
	use tcod::input::KeyCode::*;
	//handle keys
	let key = tcod.root.wait_for_keypress(true);
	match key {
	Key {
		code: Enter,
		alt: true, 
		..
		} => {
			// Alt+Enter: toggle fullscreen
			let fullscreen = tcod.root.is_fullscreen();
			tcod.root.set_fullscreen(!fullscreen);
		}
		Key { code: Escape, .. } => return true, //exit the game
		//move keys, match better than ifelses, .. means ignore others
		Key { code: Up, .. } => player.move_by(0, -1, game),
		Key { code: Down, .. } => player.move_by(0, 1, game),
		Key { code: Left, .. } => player.move_by(-1, 0, game),
		Key { code: Right, .. } => player.move_by(1, 0, game),
		
		_ => {} //match everything else	
	}
	
	false
}

////////////////////////////////////////////////////main //////////////////////////////////////////////

fn main(){
	tcod::system::set_fps(LIMIT_FPS);

	let root = Root::initializer() //new variable root console
		.font("arial10x10.png", FontLayout::Tcod) //set font
		.font_type(FontType::Greyscale) 
		.size(SCREEN_WIDTH, SCREEN_HEIGHT) 
		.title("Rust/libtcod tutorial") 
		.init();

	let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

	let mut tcod = Tcod { root, con };

    let game = Game {
        //generate map
        map: make_map(),
    };

	// obj rep player
	let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

	// npc
	let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);

	let mut objects = [player, npc];

	while !tcod.root.window_closed() { 
		tcod.con.clear(); // removes stuff we drew on prev frame
		for object in &objects {
			object.draw(&mut tcod.con);
		}

        render_all(&mut tcod, &game, &objects);
		tcod.root.flush();

		// handle keys & exit if needbe
		let player = &mut objects[0];
		let exit = handle_keys(&mut tcod, &game, player);
		if exit {
			break;
		}
	}
}
