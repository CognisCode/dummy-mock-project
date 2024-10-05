use simulation::simulation::*;

fn main() {
    nannou::app(app).update(next_step).exit(end).run();
}
