
pub struct AdaptorPDF {
    // pub name: String
}

pub trait AdaptorHtmlToPDFMake {
    fn make(&self) -> Result<(), anyhow::Error>;
}
