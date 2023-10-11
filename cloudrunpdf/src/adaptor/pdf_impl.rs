pub use usecase::adaptor::pdf::{AdaptorPDF,AdaptorHtmlToPDFMake};

use std::fs;
use headless_chrome::{Browser, types::PrintToPdfOptions};

pub struct AdaptorPDFImpl(pub AdaptorPDF);

impl AdaptorHtmlToPDFMake for AdaptorPDFImpl {
    fn make(&self) -> Result<(), anyhow::Error> {
        // println!("Value Test: ", self.0.name);
        println!("make Test1");
        let browser = Browser::default()?;
        println!("make Test2");
        let tab = browser.new_tab()?;
        println!("make Test3");
        let mut options = PrintToPdfOptions::default();
        options.scale = Some(0.7);
        options.margin_bottom = Some(0.0);
        options.margin_left = Some(0.0);
        options.margin_right = Some(0.0);
        options.margin_top = Some(0.0);
        println!("make Test4");
        let wikidata = tab
            .navigate_to("https://www.naver.com")?
            .wait_until_navigated()?
            .print_to_pdf(Some(options))?; 
        println!("make Test5");
        fs::write("wiki.pdf", wikidata)?;
        println!("PDF successfully created from internet web page.");
        Ok(())
    }
}