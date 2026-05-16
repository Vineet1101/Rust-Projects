use trpl::Html;

fn main(){
    println!("This is my first async program")
}

async fn page_title(url:&str)->Option<String>{
    let response=trpl::get(url).await;
    let response_text=response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.innner_html())
    
}
