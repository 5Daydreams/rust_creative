fn main() {
    nannou::app(texture_galaxy::model)
        .update(texture_galaxy::update)
        .view(texture_galaxy::view)
        .run();
}
