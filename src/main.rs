use std::path::PathBuf;
use actix_files as fs;
use actix_web::{get, web::ServiceConfig, Error};
use fs::NamedFile;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_static_folder::StaticFolder(folder = "assets")] _static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg
            .service(fs::Files::new("/static", "./assets").show_files_listing())
            .service(favicon_ico);
    };

    Ok(config.into())
}

#[get("/favicon.ico")]
async fn favicon_ico() -> Result<NamedFile, Error> {
    Ok(NamedFile::open(PathBuf::from("./assets/favicon.ico"))?)
}