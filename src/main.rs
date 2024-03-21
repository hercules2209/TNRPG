use std::thread;
use std::time::Duration;

fn main(){
    let mut game_state=GameState::new()
    loop{
        game_state.update();
        
        game_state.render();
        
        thread::sleep(Duration::from_millis(16));
    }
}
