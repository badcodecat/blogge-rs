#[rocket::get("/")]
fn index() -> &'static str
{
	"Hello, World!"
}

#[rocket::launch]
fn rocket() -> _
{
	use rocket::routes;
	rocket::build()
		.mount("/", routes![index])
}
