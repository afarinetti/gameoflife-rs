mod game;

fn main() {
    let mut sim = game::ConwaySim::new(5, 5);

    sim.set_cells(&[
        (2, 1),
        (2, 2),
        (2, 3)
    ]);

    for _i in 0..105 {
        sim.step();
        
        println!("Generation: {}", sim.get_generation());
        print!("{}", sim);
        println!("Any cell alive? {}", sim.is_any_cell_alive());
        println!();

        if !sim.is_any_cell_alive() {
            break;
        }

    }
}
