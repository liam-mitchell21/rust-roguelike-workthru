

//libs 
use tcod::colors::*;
use tcod::console::*;

// actual window size
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // fps max

struct Tcod { //for doing all libtcod values
	root: Root,
}

fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
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
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,
        
        _ => {} //match everything else	
    }
    
    false
}

fn main(){
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer() //new variable root console
        .font("arial10x10.png", FontLayout::Tcod) //set font
        .font_type(FontType::Greyscale) 
        .size(SCREEN_WIDTH, SCREEN_HEIGHT) 
        .title("Rust/libtcod tutorial") 
        .init();

    let mut tcod = Tcod { root };

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    while !tcod.root.window_closed() { 
        tcod.root.set_default_foreground(WHITE); //foreground color
        tcod.root.clear(); // removes stuff we drew on prev frame
        tcod.root
            .put_char(player_x, player_y, '@', BackgroundFlag::None); // draw char
        tcod.root.flush(); // draw everything at once
        
        // handle keys & exit if needbe
        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }
}
