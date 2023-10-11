pub mod adaptor;

use actix_web::web::Data;
use actix_web::{get, web, App, HttpServer, Responder, post};
use adaptor::pdf_impl::AdaptorPDFImpl;
use usecase::adaptor::pdf::AdaptorPDF;
use usecase::usecase_pdf::{UseCasePDF, PDFAdaptors};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    println!("helloTest");
    format!("Hello {}!", name)
}

#[get("/pdf")]
async fn create_pdf(
    usecase_pdf: web::Data<Box<dyn UseCasePDF>>
) -> impl Responder {
    usecase_pdf.make_pdf();
    format!("PDF Success")
}

#[post("/pdf/{name}")]
async fn export_pdf(test: web::Path<String>) -> impl Responder {
    format!("helloTest {}!", test)
}

fn init() {
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();
    // assert!(browse_wikipedia().is_ok());

    HttpServer::new(|| {
        let usecase_pdf:Box<dyn UseCasePDF> = Box::new(
            PDFAdaptors{
                adaptor_pdf:Box::new(
                    AdaptorPDFImpl(AdaptorPDF{}
                ))
            }
        );
        
        App::new()
            .app_data(Data::new(usecase_pdf))
            .service(greet)
            .service(create_pdf)
            .service(export_pdf)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
