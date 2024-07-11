#[rocket::get("/")]
fn index() -> &'static str
{
	"Hello, World!"
}

#[rocket::launch]
fn rocket() -> rocket::Rocket<rocket::Build>
{
	use rocket::routes;
	rocket::build()
		.mount("/", routes![index])
}
