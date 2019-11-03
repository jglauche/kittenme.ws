#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate haml;
use std::io::Cursor;
use rocket_contrib::templates::{Template};
use rocket::http::Method;
use rocket::fairing::AdHoc;
use serde::ser::Serialize;

#[derive(Serialize)]
struct TemplateContext {
		name: String,
		sub: String,
		label: String,
		//items: Vec<&'static str>
}

#[derive(Serialize)]
struct InnerContent {
  content: String,
}


#[get("/")]
fn index() -> Template {
	let context = TemplateContext { name: String::from("Ohai from Rocket"), sub: String::from("Serving haml via tera templates"), label: String::from("is cute") };
  render("index", "base", &context)
}

fn render(inner: &str, outer: &str, context: &impl Serialize) -> Template
{
  let local_rocket = rocket::ignite().attach(Template::fairing());

  let inner = Template::show(&local_rocket, String::from(inner), &context);
  let mut content = InnerContent{ content: String::from("") };
  let indent = 2; // FIXME

  match inner {
    Some(data) => {
      for line in data.split('\n') {
        for _ in 0..indent {
          content.content += "  ";
        }
        content.content += line;
        content.content += "\n";
      }
    },
    None => {}
  }

  Template::render(String::from(outer), &content)
}


fn main() {
		rocket::ignite()
			.mount("/", routes![index])
			.attach(Template::fairing())
			.attach(AdHoc::on_response("haml", |req, response| {
				if req.method() == Method::Get{
					match response.body_string(){
						Some(body) => {
							println!("{:?}",body);
							response.set_sized_body(Cursor::new(haml::to_html(&body)));
						},
						None => {}
					}

				}
			}))
			.launch();
}
