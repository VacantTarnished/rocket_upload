use rocket::{launch, routes, post, fs::TempFile, form::FromForm, form::Form, http::Status};
use regex::Regex;

#[derive(FromForm)]
struct Upload<'f> {
    file: TempFile<'f>
}

#[post("/", format = "multipart/form-data", data = "<form>")]
async fn upload_file(mut form: Form<Upload<'_>>) -> Result<(), Status> {
    let new_regex_unsafe = std::env::var("UPLOAD_FILE_REGEX");
    
    if new_regex_unsafe.is_err() {
        return Err(Status::InternalServerError);
    }

    let new_regex = Regex::new(new_regex_unsafe.unwrap().as_str());

    if new_regex.is_err() {
        return Err(Status::InternalServerError);
    }

    let re = new_regex.unwrap();

    if form.file.raw_name().is_none() {
        return Err(Status::BadRequest);
    }


    let file_name = form.file.raw_name().unwrap().dangerous_unsafe_unsanitized_raw().to_string();
    if re.is_match(file_name.as_str()) {
        if form.file.copy_to(file_name).await.is_ok() {
            let status_result = std::process::Command::new("/post-upload").status();
            
            if status_result.is_err() {
                return Err(Status::InternalServerError);
            }
            
            let status = status_result.unwrap();

            if status.success() {
                Err(Status::Created)
            } else {
                Err(Status::BadRequest)
            }
        } else {
            Err(Status::InternalServerError)
        }
    } else {
        Err(Status::BadRequest)
    }

    
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload_file])
}
