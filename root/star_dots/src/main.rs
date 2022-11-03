fn main()
{
	nannou::app(star_dots::model)
	.update(star_dots::update)
		.view(star_dots::view)
		.run();
}
