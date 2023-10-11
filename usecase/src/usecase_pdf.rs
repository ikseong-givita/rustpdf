use crate::adaptor::pdf::AdaptorHtmlToPDFMake;

pub trait UseCasePDF {
    fn make_pdf(&self);
}

pub struct PDFAdaptors {
    pub adaptor_pdf: Box<dyn AdaptorHtmlToPDFMake>
}

impl UseCasePDF for PDFAdaptors {
    fn make_pdf(&self){
        let _ = self.adaptor_pdf.make();
    }
}