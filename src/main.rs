use thirtyfour::{error::WebDriverError, By, DesiredCapabilities, WebDriver};
use std::{error::Error, thread, time::Duration};

const PAGE: &str = "https://www.scrapingcourse.com/ecommerce/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let driver = initialize_driver().await?;

    let first_page = parse_page(&driver, PAGE).await?;

    first_page.products.iter().for_each(|p| println!("Name: {} | Price: {} | Img: {} | Page: {}", p.name, p.price, p.img_url, p.page));

    Ok(())
}

async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.maximize_window().await?;
    Ok(driver)
}

struct PageData {
    products: Vec<Product>
}

struct Product {
    name: String,
    page: String,
    img_url: String,
    price: String
}

async fn parse_page(driver: &WebDriver, page: &str) -> Result<PageData, WebDriverError> {
    driver.goto(page).await?;
    thread::sleep(Duration::from_secs(2));
    let product_elements = driver.find_all(By::ClassName("product")).await?;
    let mut products = vec![];

    for elem in &product_elements {
        let e = elem.find(By::ClassName("product-name")).await.expect("No product-name").text().await?;
        let p = elem.find(By::Tag("bdi")).await?.text().await?;
        let product = Product {
            name: e,
            price: p,
            img_url: elem.find(By::Tag("img")).await?.attr("src").await?.unwrap(),
            page: driver.current_url().await?.to_string()
        };
        products .push(product);
    }

    products.iter().for_each(|p| println!("Name: {} | Price: {} | Img: {} | Page: {}", p.name, p.price, p.img_url, p.page));
    Ok(PageData { products })
}
