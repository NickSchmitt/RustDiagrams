// Handlebars
use rocket_contrib::templates::Template;
use std::collections::HashMap;

// queries
use diesel::prelude::*;

// macros
use crate::schema::*;

// structs
use crate::models::*;

// parsing
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

// flash
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};


#[get("/")]
pub fn list(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();

    let diagrams: Vec<Diagram> = diagrams::table
        .select(diagrams::all_columns)
        .load::<Diagram>(&crate::establish_connection())
        .expect("failed to establish connection");

    if let Some(ref msg) = flash {
        context.insert("data", (diagrams, msg.msg()));
    } else {
        context.insert("data", (diagrams, "Displaying diagrams"));
    }
    println!("{:?}", &context);
    Template::render("list", &context)
}

#[get("/new")]
pub fn new(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }
    Template::render("new", context)
}

#[post("/insert", data = "<diagram_data>")]
pub fn insert(content_type: &ContentType, diagram_data: Data) -> Flash<Redirect> {
    use std::fs;

    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::file("photo"),
        MultipartFormDataField::text("title"),
        MultipartFormDataField::text("caption"),
    ];

    let multipart_form_data = MultipartFormData::parse(content_type, diagram_data, options);

    match multipart_form_data {
        Ok(form) => {

            let diagram_img = match form.files.get("photo") {
                Some(img) => {
                    let file_field = &img[0];
                    let _content_type = &file_field.content_type;
                    let _file_name = &file_field.file_name;
                    let _path = &file_field.path;

                    let file_format: Vec<&str> = _file_name.as_ref().unwrap().split('.').collect();

                    let absolute_path: String = format!("img/{}", _file_name.clone().unwrap());
                    fs::copy(_path, &absolute_path).unwrap();

                    Some(format!("img/{}", _file_name.clone().unwrap()))
                }
                None => None,
            };

            let insert = diesel::insert_into(diagrams::table)
                .values(NewDiagram {
                    title: match form.texts.get("title") {
                        Some(value) => &value[0].text,
                        None => "No Title.",
                    },
                    photo: diagram_img.unwrap(),
                    caption: match form.texts.get("caption") {
                        Some(caption) => &caption[0].text,
                        None => "No Caption.",
                    },
                })
                .execute(&crate::establish_connection());

            match insert {
                Ok(_) => {
                    Flash::success(Redirect::to("/"), "Success! New diagram added to database.")
                }
                Err(err_msg) => Flash::error(
                    Redirect::to("/new"),
                    format!("Database insertion error: {}", err_msg),
                ),
            }
        }

        Err(err_msg) => Flash::error(
            Redirect::to("/new"),
            format!("Form parsing error: {}", err_msg),
        ),
    }
}

#[get("/update/<id>")]
pub fn update(id: i32) -> Template {
    let mut context = HashMap::new();
    let diagram_data = diagrams::table
        .select(diagrams::all_columns)
        .filter(diagrams::id.eq(id))
        .load::<Diagram>(&crate::establish_connection())
        .expect("An error occurred while retrieving the diagram.");

    context.insert("diagram", diagram_data);

    Template::render("update", &context)
}

#[post("/update", data = "<diagram_data>")]
pub fn process_update(content_type: &ContentType, diagram_data: Data) -> Flash<Redirect> {
    use std::fs;

    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::file("photo"),
        MultipartFormDataField::text("id"),
        MultipartFormDataField::text("title"),
        MultipartFormDataField::text("caption"),
    ];

    let multipart_form_data = MultipartFormData::parse(content_type, diagram_data, options);

    match multipart_form_data {
        Ok(form) => {
            let diagram_img = match form.files.get("photo") {
                Some(img) => {
                    let file_field = &img[0];
                    let _content_type = &file_field.content_type;
                    let _file_name = &file_field.file_name;
                    let _path = &file_field.path;

                    let file_format: Vec<&str> = _file_name.as_ref().unwrap().split('.').collect();

                    let absolute_path: String = format!("img/{}", _file_name.clone().unwrap());
                    fs::copy(_path, &absolute_path).unwrap();

                    Some(format!("img/{}", _file_name.clone().unwrap()))
                }
                None => None,
            };

            let insert = diesel::update(
                diagrams::table.filter(
                    diagrams::id.eq(form.texts.get("id").unwrap()[0]
                        .text
                        .parse::<i32>()
                        .unwrap()),
                ),
            )
            .set(NewDiagram {
                title: match form.texts.get("title") {
                    Some(value) => &value[0].text,
                    None => "No Title.",
                },
                photo: diagram_img.unwrap(),
                caption: match form.texts.get("caption") {
                    Some(value) => &value[0].text,
                    None => "No Caption.",
                },
            })
            .execute(&crate::establish_connection());

            match insert {
                Ok(_) => Flash::success(Redirect::to("/"), "Diagram updated."),
                Err(err_msg) => Flash::error(
                    Redirect::to("/new"),
                    format!("An error occurred while updating the diagram: {}", err_msg),
                ),
            }
        }
        Err(err_msg) => {
            Flash::error(
                Redirect::to("/new"),
                format!("Database update error: {}", err_msg),
            )
        }
    }
}

#[get("/delete/<id>")]
pub fn delete(id: i32) -> Flash<Redirect> {
    diesel::delete(diagrams::table.filter(diagrams::id.eq(id)))
        .execute(&crate::establish_connection())
        .expect("Deletion error.");
    Flash::success(Redirect::to("/"), "Diagram successfully deleted.")
}
